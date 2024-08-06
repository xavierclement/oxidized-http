// Copyright (c) 2024 XAVIER Clément Antoine
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of
// this software and associated documentation files (the "Software"), to deal in
// the Software without restriction, including without limitation the rights to
// use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
// the Software, and to permit persons to whom the Software is furnished to do so,
// subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
// FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
// COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
// IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Number of workers
    #[arg(short, long, env, default_value_t = 1)]
    count: u32,
}

struct Worker {
    id: u32, // Worker ID
}

impl Worker {
    fn new(id: u32) -> Worker {
        Worker { id }
    }

    fn run(&self) {
        println!("Worker {} started", self.id);

        for i in 0..10 {
            println!("Worker {} is working... {}", self.id, i);
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    }
}


fn main() {
    let args = Args::parse();

    println!("Number of workers: {}", args.count);

    let mut handles: Vec<std::thread::JoinHandle<()>> = Vec::new();

    // Launch the workers in separate threads
    for i in 0..args.count {
        let worker = Worker::new(i);
        let handle = std::thread::spawn(move || {
            worker.run();
        });
        handles.push(handle);
    }

    // Wait for all workers to finish
    for handle in handles {
        handle.join().unwrap();
    }

    println!("All workers finished");
}
