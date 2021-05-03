use std::collections::VecDeque;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Write;
use std::net::TcpListener;
use std::net::TcpStream;
use std::sync::Arc;
use std::sync::Mutex;
// debugging in Rust
#[derive(Debug, Eq, PartialEq)]

enum Request {
    Publish(String),
    Retrieve,
}
fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    // unwrap -> If this doesn't work then the proogram should shut down in a structured way

    // arc = atomic ref counter (reference counting is done atomically)
    let mut storage = Arc::new(Mutex::new(VecDeque::new()));

    for connection_attempt in listener.incoming() {
        // allows us to look at these result/similar types
        match connection_attempt {
            Ok(stream) => {
                // moving ownership to a thread in an iteration, we llosee data after one iteration
                // Ans: we can trry to reference it? -> It doesn't work
                //let ref_storage = &mut storage;
                // ^ this also wwon't work aas we dunno how long the thread runs
                let mut thread_handle = Arc::clone(&storage);

                std::thread::spawn(move || {
                    handle_client(stream, &mut thread_handle);
                });
            }
            Err(e) => {
                // prints in standard error
                eprintln!("Error connecting {}", e);
            }
        }
    }
}

fn handle_client(mut stream: TcpStream, storage: &Mutex<VecDeque<String>>) -> () {
    // inbuilt func which crashes to do stuff later
    // todo()
    let line = read_line(&stream);

    let request = parse_request(line);

    match request {
        Request::Publish(msg) => {
            eprintln!("publishing message");
            // we can only access storage throuhg lock in a Mutex
            // This is called borrow checker ig?
            // Rust gives 3 levels of checking here, being Mutex, Keep a lock
            // and only access data till lock is there
            // math behind it -> https://plv.mpi-sws.org/rustbelt/
            let mut guard = storage.lock().unwrap();
            guard.push_back(msg);
            // mutex allows only one guard at once?
            // guard is released end of the scope
        }
        Request::Retrieve => {
            let mut guard = storage.lock().unwrap();
            let maybe_msg = guard.pop_front();
            match maybe_msg {
                Some(msg) => {
                    stream.write_all(msg.as_bytes()).unwrap();
                }
                None => {
                    stream.write_all(b"no message available").unwrap();
                }
            }
            eprintln!("retriving message");
        }
    }

    println!("Client connnected !! ");
}

fn parse_request(line: String) -> Request {
    let trimmed = line.trim_end();

    if trimmed == "" {
        Request::Retrieve
    } else {
        // creating a string as we may be accessing a string which is delocated?
        Request::Publish(String::from(trimmed))
    }
}
fn read_line(stream: &TcpStream) -> String {
    let mut buffered_reader = BufReader::new(stream);

    let mut buf = String::new();
    // Rust has 2 difference references for mutable and immutable. Also, they can't be referenced at once
    buffered_reader.read_line(&mut buf).unwrap();
    buf
}
