use std::{
    collections::VecDeque,
    fs::File,
    io::Read,
    ops::Range,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

trait Document<'a>: Iterator {
    fn change(range: &Range<usize>, with: &str);
}

struct ConcurrentDocument {
    queque: VecDeque<String>,
}
impl ConcurrentDocument {
    pub fn new() -> Self {
        Self {
            queque: VecDeque::new(),
        }
    }
    pub fn add(&mut self, text: String) {
        self.queque.push_back(text);
    }
    pub fn iter(&self) -> std::collections::vec_deque::Iter<String> {
        self.queque.iter()
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
            doc.add(String::from(str));

            thread::sleep(Duration::from_millis(100));
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    for x in arc.lock().unwrap().iter() {
        println!("{}", x);
    }
}
