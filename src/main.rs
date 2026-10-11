#![forbid(unsafe_code)]

mod bounded_input;
mod worker;
mod worker_io;
mod worker_task;

fn main() {
    if let Err(error) = worker::run() {
        eprintln!("hat-hibee-operator-worker: {error}");
        std::process::exit(1);
    }
}
