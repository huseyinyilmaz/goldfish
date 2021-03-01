mod handler;
// use std::io;
use std::net;
use std::thread;
use handler::Handler;


pub fn run() {
    let mut threads = Vec::new();
    let listener = net::TcpListener::bind("127.0.0.1:3306").unwrap();

    while let Ok((s, _)) = listener.accept() {
        threads.push(thread::spawn(move || {
            let handler = Handler::new(s);
            println!("handler: {:?}", handler);
        }));
    }

    for t in threads {
        t.join().unwrap();
    }
}
