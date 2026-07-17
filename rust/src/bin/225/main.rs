use std::collections::VecDeque;

struct MyStack {
    queue: VecDeque<i8>,
}

impl MyStack {
    fn new() -> Self {
        MyStack {
            queue: Default::default(),
        }
    }

    fn push(&mut self, x: i32) {
        self.queue.push_back(x as i8);
    }

    fn pop(&mut self) -> i32 {
        for _ in 0..self.queue.len() - 1 {
            let v = self.queue.pop_front().unwrap();
            self.queue.push_back(v);
        }

        self.queue.pop_front().unwrap() as i32
    }

    fn top(&mut self) -> i32 {
        for _ in 0..self.queue.len() - 1 {
            let v = self.queue.pop_front().unwrap();
            self.queue.push_back(v);
        }

        let result = self.queue.pop_front().unwrap();
        self.queue.push_back(result);

        result as i32
    }

    fn empty(&self) -> bool {
        self.queue.is_empty()
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
