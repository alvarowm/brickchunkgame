use std::sync::{Arc, Mutex};
use std::thread;
use crossterm::event::{read, Event, KeyEvent};

pub fn read_input(mutex: &Arc<Mutex<Vec<KeyEvent>>>) {
    let clone = Arc::clone(mutex);
    thread::spawn(move || {
        loop {
            if let Event::Key(key) = read().unwrap() { clone.lock().unwrap().push(key) };
        }
    });
}
