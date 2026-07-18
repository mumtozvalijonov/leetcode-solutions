use std::collections::VecDeque;

struct MyQueue {
    push_stack: VecDeque<i32>,
    pop_stack: VecDeque<i32>,
}

impl MyQueue {
    fn new() -> Self {
        MyQueue {
            push_stack: Default::default(),
            pop_stack: Default::default(),
        }
    }

    fn push(&mut self, x: i32) {
        self.push_stack.push_back(x);
    }

    fn pop(&mut self) -> i32 {
        if let Some(v) = self.pop_stack.pop_back() {
            return v;
        }

        while let Some(v) = self.push_stack.pop_back() {
            self.pop_stack.push_back(v);
        }

        self.pop()
    }

    fn peek(&mut self) -> i32 {
        if let Some(v) = self.pop_stack.back() {
            return *v;
        }

        while let Some(v) = self.push_stack.pop_back() {
            self.pop_stack.push_back(v);
        }

        self.peek()
    }

    fn empty(&self) -> bool {
        self.pop_stack.is_empty() && self.push_stack.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut queue = MyQueue::new();
        queue.push(1);
        queue.push(2);

        assert_eq!(queue.peek(), 1);
        assert_eq!(queue.pop(), 1);
        assert!(!queue.empty());

        assert_eq!(queue.peek(), 2);
        assert_eq!(queue.pop(), 2);
        assert!(queue.empty());
    }
}
