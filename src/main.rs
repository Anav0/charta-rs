use std::{
    collections::VecDeque,
    fs::File,
    io::{Read, Write},
    ops::Range,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

trait Document<'a>: Iterator {
    fn change(range: &Range<usize>, with: &str);
}

struct ConcurrentDocument<'a> {
    reader: &'a dyn Read,
    writer: &'a dyn Write,
}

impl<'a> ConcurrentDocument<'a> {
    pub fn new(reader: &'a mut dyn Read, writer: &'a mut dyn Write) -> Self {
        Self { reader, writer }
    }
}

fn main() {
    let arc = Arc::new(Mutex::from(ConcurrentDocument::new()));
    let mut handles = vec![];

    for j in 1..10 {
        let mutex = Arc::clone(&arc);
        let handle = thread::spawn(move || {
            let mut doc = mutex.lock().expect("Failed to lock");
            let str = format!("Thread {}", j);

            thread::sleep(Duration::from_millis(100));
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
