use std::mem::MaybeUninit;

fn main() {}

struct MinStack {
    nodes: [MaybeUninit<(i32, i32)>; 30000],
    cursor: usize,
}

impl MinStack {
    fn new() -> Self {
        MinStack {
            nodes: [MaybeUninit::uninit(); 30000],
            cursor: 0,
        }
    }

    fn push(&mut self, value: i32) {
        let new_node = if self.cursor == 0 {
            (value, value)
        } else {
            (value, unsafe {
                value.min((&*self.nodes[self.cursor - 1].as_ptr()).1)
            })
        };

        self.nodes[self.cursor].write(new_node);
        self.cursor += 1;
    }

    fn pop(&mut self) {
        self.cursor -= 1;
    }

    fn top(&self) -> i32 {
        unsafe { (&*self.nodes[self.cursor - 1].as_ptr()).0 }
    }

    fn get_min(&self) -> i32 {
        unsafe { (&*self.nodes[self.cursor - 1].as_ptr()).1 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut ms = MinStack::new();

        ms.push(-2);
        ms.push(0);
        ms.push(-3);

        assert_eq!(ms.get_min(), -3);
        ms.pop();
        assert_eq!(ms.top(), 0);
        assert_eq!(ms.get_min(), -2);
    }
}
