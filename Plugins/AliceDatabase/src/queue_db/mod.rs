use std::collections::VecDeque;
use std::sync::Mutex;

#[derive(Debug)]
pub struct Command {
    pub command: String,
    pub author: String,
}

impl Command {
    pub fn new(command: &str, author: &str) -> Self {
        Command { command: command.to_string(), author: author.to_string() }
    }
}
#[derive(Debug)]
pub struct Queue {
    pub deque: Mutex<VecDeque<Command>>,
}

impl Queue {
    pub fn new() -> Self {
        Queue {
            deque: Mutex::new(VecDeque::new()),
        }
    }

    pub fn enqueue(&self, command: Command) {
        let mut deque = self.deque.lock().unwrap();
        deque.push_back(command);
    }

    pub fn dequeue(&self) -> Option<Command> {
        let mut deque = self.deque.lock().unwrap();
        deque.pop_front()
    }

    pub fn is_empty(&self) -> bool {
        let deque = self.deque.lock().unwrap();
        deque.is_empty()
    }

    pub fn size(&self) -> usize {
        let deque = self.deque.lock().unwrap();
        deque.len()
    }
}