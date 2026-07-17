use std::collections::VecDeque;

struct MyStack {
    pop_queue: VecDeque<i32>,
    push_queue: VecDeque<i32>,
}

impl MyStack {
    fn new() -> Self {
        MyStack {
            pop_queue: Default::default(),
            push_queue: Default::default(),
        }
    }

    fn push(&mut self, x: i32) {
        while let Some(v) = self.pop_queue.pop_front() {
            self.push_queue.push_back(v);
        }

        self.pop_queue.push_back(x);
    }

    fn pop(&mut self) -> i32 {
        if let Some(v) = self.pop_queue.pop_front() {
            return v;
        }

        while let Some(v) = self.push_queue.pop_front() {
            self.pop_queue.push_back(v);
        }

        for _ in 0..self.pop_queue.len() - 1 {
            self.push_queue
                .push_back(self.pop_queue.pop_front().unwrap());
        }

        self.pop()
    }

    fn top(&mut self) -> i32 {
        if let Some(v) = self.pop_queue.front() {
            return *v;
        }

        while let Some(v) = self.push_queue.pop_front() {
            self.pop_queue.push_back(v);
        }

        for _ in 0..self.pop_queue.len() - 1 {
            self.push_queue
                .push_back(self.pop_queue.pop_front().unwrap());
        }

        self.top()
    }

    fn empty(&self) -> bool {
        self.pop_queue.is_empty() && self.push_queue.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut stack = MyStack::new();
        stack.push(1);
        stack.push(2);

        assert_eq!(stack.top(), 2);
        assert_eq!(stack.pop(), 2);
        assert!(!stack.empty());
        assert_eq!(stack.top(), 1);
        assert_eq!(stack.pop(), 1);
        assert!(stack.empty());
    }
}
