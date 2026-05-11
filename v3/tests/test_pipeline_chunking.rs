//! Tests that pipelined commands survive TCP chunking boundaries.
//!
//! When TCP delivers data in chunks, a storage command's inline data
//! may be split across two reads.  The server must buffer the incomplete
//! command and resume parsing once the rest arrives — it must NOT let the
//! catch-all `CannotParse` consume the remaining bytes (which destroys
//! subsequent well-formed commands).
//!
//! This test simulates two TCP reads by splitting a batch at an
//! arbitrary byte offset that falls within a SET command's data,
//! feeding each chunk to `process_input` separately.
//!
//! ## Current behaviour (bug)
//!
//! `CannotParse` uses `nom::combinator::rest` and eats every remaining
//! byte — including the rest of the split command AND every well-formed
//! command that follows it.  Keys past the split point are permanently
//! lost.
//!
//! ## Expected behaviour (after fix)
//!
//! The server holds a persistent `Vec<u8>` buffer.  `process_input`
//! should only consume complete commands; incomplete data stays in the
//! buffer and is retried once more data arrives.

mod common;

const VALUE_SIZE: usize = 4096;
const VALUE_DATA: [u8; VALUE_SIZE] = [b'x'; VALUE_SIZE];

fn make_set_cmd(key: usize) -> Vec<u8> {
    let header = format!("set key{key} 0 10 {VALUE_SIZE}\r\n");
    let header_bytes = header.as_bytes();
    let mut cmd = Vec::with_capacity(header_bytes.len() + VALUE_SIZE + 2);
    cmd.extend_from_slice(header_bytes);
    cmd.extend_from_slice(&VALUE_DATA);
    cmd.extend_from_slice(b"\r\n");
    cmd
}

fn make_get_cmd(key: usize) -> Vec<u8> {
    format!("get key{key}\r\n").into_bytes()
}

/// Known-good baseline: a single process_input call with the whole batch.
#[test]
fn test_baseline_whole_batch_works() {
    let state = common::new_state();

    let mut batch = Vec::new();
    for i in 0..32 {
        batch.extend_from_slice(&make_set_cmd(i));
    }

    let mut output = Vec::new();
    goldfish::process_input(&state, &batch, &mut output);

    let out = String::from_utf8_lossy(&output);
    assert_eq!(out.matches("STORED").count(), 32, "whole batch baseline");
    assert!(!out.contains("CLIENT_ERROR"), "baseline had CLIENT_ERROR");
    assert!(!out.contains("ERROR"), "baseline had ERROR");

    for i in 0..32 {
        let mut get_out = Vec::new();
        goldfish::process_input(&state, &make_get_cmd(i), &mut get_out);
        assert!(
            get_out.starts_with(b"VALUE"),
            "key{i} missing in baseline — this should never happen"
        );
    }
}

/// Demonstrates the TCP-chunking bug.
///
/// Splits the batch mid-data, feeds each chunk through separate
/// `process_input` calls.  With the current implementation the
/// catch-all `CannotParse` consumes the partial data AND subsequent
/// commands, so keys after the split point are lost.
///
/// This test FAILS on the current code and will PASS after the
/// persistent-buffer fix is applied.
#[test]
fn test_pipeline_survives_tcp_chunking() {
    let state = common::new_state();

    // Build 32 SET commands with 4 KB values (~132 KB total).
    let mut batch = Vec::new();
    for i in 0..32 {
        batch.extend_from_slice(&make_set_cmd(i));
    }
    assert!(batch.len() > 65_000, "batch must be large enough to split");

    // Simulate two TCP reads: split at 65 536 bytes.
    // This offset falls inside the data portion of key~15.
    let split = 65_536;
    let (chunk1, chunk2) = batch.split_at(split);

    // First "read" ── processes ~15 complete commands + a split command tail.
    let mut out1 = Vec::new();
    goldfish::process_input(&state, chunk1, &mut out1);

    // Second "read" ── receives the rest of the split data + remaining commands.
    let mut out2 = Vec::new();
    goldfish::process_input(&state, chunk2, &mut out2);

    // Combine all responses.
    let mut all_output = out1;
    all_output.extend_from_slice(&out2);
    let output_str = String::from_utf8_lossy(&all_output);
    let stored = output_str.matches("STORED").count();
    let client_errors = output_str.matches("CLIENT_ERROR").count();
    let errors = output_str.matches("ERROR").count();

    // ── This assertion fails with the current code ────────────────
    // CannotParse eats everything past the split, so only ~15 keys
    // survive.  After the fix all 32 should be stored.
    assert_eq!(
        stored, 32,
        "\n\
         ───────────────────────────────────────────────────────\n\
         TCP-CHUNKING BUG REPRODUCED\n\
         ───────────────────────────────────────────────────────\n\
         Expected 32 STORED responses but got {stored}.\n\
         CLIENT_ERROR count: {client_errors}\n\
         ERROR count:         {errors}\n\
         \n\
         This is the KNOWN PIPELINING BUG: the catch-all CannotParse\n\
         consumed subsequent well-formed commands after the split.\n\
         \n\
         Full output:\n\
         {output_str}\n\
         ───────────────────────────────────────────────────────"
    );

    // Also verify every key is actually retrievable.
    for i in 0..32 {
        let mut get_out = Vec::new();
        goldfish::process_input(&state, &make_get_cmd(i), &mut get_out);
        assert!(
            get_out.starts_with(b"VALUE"),
            "key{i} was lost — GET returned {got:?}",
            got = String::from_utf8_lossy(&get_out)
        );
    }
}
