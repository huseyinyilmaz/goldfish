use criterion::{criterion_group, criterion_main, BatchSize, Criterion};
use goldfish::process_input;
use goldfish::state::State;
use std::sync::{Arc, Mutex};
use std::time::Duration;

const TOTAL_OPS: usize = 10000;
const CLIENTS: usize = 100;
const COMMANDS_PER_CLIENT: usize = TOTAL_OPS / CLIENTS;
const VALUE_SIZE: usize = 4096;

fn make_set_cmd(key: usize) -> Vec<u8> {
    let header = format!("set key{} 0 10 {}\r\n", key, VALUE_SIZE);
    let header_bytes = header.as_bytes();
    let mut cmd = Vec::with_capacity(header_bytes.len() + VALUE_SIZE + 2);
    cmd.extend_from_slice(header_bytes);
    cmd.extend(std::iter::repeat(b'x').take(VALUE_SIZE));
    cmd.extend_from_slice(b"\r\n");
    cmd
}

fn make_get_cmd(key: usize) -> Vec<u8> {
    format!("get key{}\r\n", key).into_bytes()
}

fn build_batch(base_key: usize, count: usize, write_pct: usize) -> Vec<u8> {
    let mut cmds = Vec::new();
    for i in 0..count {
        if i % 100 < write_pct {
            cmds.extend_from_slice(&make_set_cmd(base_key + i));
        } else {
            cmds.extend_from_slice(&make_get_cmd(base_key + i));
        }
    }
    cmds
}

fn pre_populate(state: &Arc<Mutex<State>>, n: usize) {
    for i in 0..n {
        let mut output = Vec::new();
        process_input(state, &make_set_cmd(i), &mut output);
    }
}

// --- Single-threaded benchmarks ---

fn st_write_heavy(c: &mut Criterion) {
    c.bench_function("st_write_heavy_5p_read_95p_write", |b| {
        b.iter_batched(
            || {
                let state = Arc::new(Mutex::new(State::new()));
                let cmds = build_batch(0, TOTAL_OPS, 95);
                (state, cmds)
            },
            |(state, cmds)| {
                let mut output = Vec::new();
                process_input(&state, &cmds, &mut output);
            },
            BatchSize::SmallInput,
        )
    });
}

fn st_read_heavy(c: &mut Criterion) {
    c.bench_function("st_read_heavy_95p_read_5p_write", |b| {
        b.iter_batched(
            || {
                let state = Arc::new(Mutex::new(State::new()));
                pre_populate(&state, TOTAL_OPS);
                let cmds = build_batch(TOTAL_OPS, TOTAL_OPS, 5);
                (state, cmds)
            },
            |(state, cmds)| {
                let mut output = Vec::new();
                process_input(&state, &cmds, &mut output);
            },
            BatchSize::SmallInput,
        )
    });
}

fn st_balanced(c: &mut Criterion) {
    c.bench_function("st_balanced_50p_read_50p_write", |b| {
        b.iter_batched(
            || {
                let state = Arc::new(Mutex::new(State::new()));
                let mut cmds = Vec::new();
                for i in 0..TOTAL_OPS / 2 {
                    cmds.extend_from_slice(&make_set_cmd(i));
                    cmds.extend_from_slice(&make_get_cmd(i));
                }
                (state, cmds)
            },
            |(state, cmds)| {
                let mut output = Vec::new();
                process_input(&state, &cmds, &mut output);
            },
            BatchSize::SmallInput,
        )
    });
}

// --- Multi-threaded benchmarks ---

fn mt_write_heavy(c: &mut Criterion) {
    c.bench_function("mt_write_heavy_5p_read_95p_write", |b| {
        b.iter_batched(
            || {
                let state = Arc::new(Mutex::new(State::new()));
                let batches: Vec<Vec<u8>> = (0..CLIENTS)
                    .map(|c| build_batch(c * COMMANDS_PER_CLIENT, COMMANDS_PER_CLIENT, 95))
                    .collect();
                (state, batches)
            },
            |(state, batches)| {
                std::thread::scope(|s| {
                    for cmds in &batches {
                        s.spawn(|| {
                            let mut output = Vec::new();
                            process_input(&state, cmds, &mut output);
                        });
                    }
                });
            },
            BatchSize::SmallInput,
        )
    });
}

fn mt_read_heavy(c: &mut Criterion) {
    c.bench_function("mt_read_heavy_95p_read_5p_write", |b| {
        b.iter_batched(
            || {
                let state = Arc::new(Mutex::new(State::new()));
                pre_populate(&state, TOTAL_OPS);
                let batches: Vec<Vec<u8>> = (0..CLIENTS)
                    .map(|c| {
                        build_batch(TOTAL_OPS + c * COMMANDS_PER_CLIENT, COMMANDS_PER_CLIENT, 5)
                    })
                    .collect();
                (state, batches)
            },
            |(state, batches)| {
                std::thread::scope(|s| {
                    for cmds in &batches {
                        s.spawn(|| {
                            let mut output = Vec::new();
                            process_input(&state, cmds, &mut output);
                        });
                    }
                });
            },
            BatchSize::SmallInput,
        )
    });
}

fn mt_balanced(c: &mut Criterion) {
    c.bench_function("mt_balanced_50p_read_50p_write", |b| {
        b.iter_batched(
            || {
                let state = Arc::new(Mutex::new(State::new()));
                let batches: Vec<Vec<u8>> = (0..CLIENTS)
                    .map(|c| {
                        let mut cmds = Vec::new();
                        for i in 0..COMMANDS_PER_CLIENT / 2 {
                            cmds.extend_from_slice(&make_set_cmd(c * COMMANDS_PER_CLIENT + i));
                            cmds.extend_from_slice(&make_get_cmd(c * COMMANDS_PER_CLIENT + i));
                        }
                        cmds
                    })
                    .collect();
                (state, batches)
            },
            |(state, batches)| {
                std::thread::scope(|s| {
                    for cmds in &batches {
                        s.spawn(|| {
                            let mut output = Vec::new();
                            process_input(&state, cmds, &mut output);
                        });
                    }
                });
            },
            BatchSize::SmallInput,
        )
    });
}

fn criterion_config() -> Criterion {
    Criterion::default()
        .sample_size(50)
        .measurement_time(Duration::from_secs(30))
        .warm_up_time(Duration::from_secs(3))
}

criterion_group!(
    name = benches;
    config = criterion_config();
    targets = st_write_heavy, st_read_heavy, st_balanced, mt_write_heavy, mt_read_heavy, mt_balanced,
);
criterion_main!(benches);
