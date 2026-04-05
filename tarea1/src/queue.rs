//! Queue implementation as linked list to support FIFO operations.
//! Uses a smart `Box` pointer to store nodes on the heap.

use super::node::Node;

/// FIFO Queue
pub struct Queue<T> {
    head: Option<Box<Node<T>>>,
    length: usize,
}

impl<T> Queue<T> {
    /// Creates a new empty queue.
    pub fn new() -> Self {
        Self {
            head: None,
            length: 0,
        }
    }

    /// Adds an item to the back of the queue.
    /// Time complexity: O(n)
    /// # Arguments
    /// * `value` - The value to push.
    pub fn push(&mut self, value: T) {
        let new_node = Box::new(Node::new(value, None));
        let mut temp = &mut self.head;
        while let Some(node) = temp {
            temp = &mut node.next;
        }
        *temp = Some(new_node);
        self.length += 1;
    }

    /// Removes the front item from the queue and returns its value.
    /// Time complexity: O(1)
    /// # Returns
    /// * `Some(T)` - The front value if the queue is not empty.
    /// * `None` - If the queue is empty.
    pub fn pop(&mut self) -> Option<T> {
        if let Some(node) = self.head.take() {
            self.head = node.next;
            self.length -= 1;
            return Some(node.value);
        }

        None
    }

    /// Returns a reference to the front item without removing it.
    /// Time complexity: O(1)
    /// # Returns
    /// * `Some(&T)` - A reference to the front value if the queue is not empty.
    /// * `None` - If the queue is empty.
    pub fn peek(&self) -> Option<&T> {
        if let Some(node) = &self.head {
            return Some(&node.value);
        }
        None
    }

    /// Returns the number of items in the queue.
    /// Time complexity: O(1)
    pub fn length(&self) -> usize {
        return self.length;
    }

    /// Checks whether the queue is empty.
    /// Time complexity: O(1)
    pub fn is_empty(&self) -> bool {
        return self.length == 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let q: Queue<i32> = Queue::new();

        assert_eq!(q.length(), 0);
        assert!(q.is_empty());
        assert_eq!(q.peek(), None);
    }

    #[test]
    fn test_push() {
        let mut q: Queue<i32> = Queue::new();
        q.push(2);
        q.push(3);

        assert!(!q.is_empty());
        assert_eq!(q.length(), 2);
        assert_eq!(q.peek(), Some(&2));
    }

    #[test]
    fn test_pop() {
        let mut q: Queue<char> = Queue::new();
        q.push('a');
        q.push('z');

        assert_eq!(q.pop(), Some('a'));
        assert_eq!(q.pop(), Some('z'));
        assert_eq!(q.pop(), None);
    }

    #[test]
    fn test_peek() {
        let mut q: Queue<String> = Queue::new();
        q.push("Hello".to_string());
        q.push("World".to_string());
        let front_string = "Hello".to_string();

        assert_eq!(q.peek(), Some(&front_string));
        assert_eq!(q.length(), 2);
        assert_eq!(q.pop(), Some(front_string));
        assert_eq!(q.peek(), Some(&"World".to_string()));
        assert_eq!(q.pop(), Some("World".to_string()));
        assert_eq!(q.peek(), None);
    }

    #[test]
    fn test_length() {
        let mut q: Queue<bool> = Queue::new();

        assert_eq!(q.length(), 0);
        q.push(true);
        assert_eq!(q.length(), 1);
        q.pop();
        assert_eq!(q.length(), 0);
        q.pop();
        assert_eq!(q.length(), 0);
    }

    #[test]
    fn test_is_empty() {
        let mut q: Queue<i32> = Queue::new();
        assert!(q.is_empty());
        q.push(1);
        assert!(!q.is_empty());
        q.pop();
        assert!(q.is_empty());
        q.pop();
        assert!(q.is_empty());
    }

    #[test]
    fn test_sequence() {
        let mut q: Queue<i32> = Queue::new();
        assert_eq!(q.length(), 0);

        q.push(10);
        assert_eq!(q.peek(), Some(&10));
        assert_eq!(q.length(), 1);

        q.push(20);
        assert_eq!(q.peek(), Some(&10));
        assert_eq!(q.length(), 2);

        assert_eq!(q.pop(), Some(10));
        assert_eq!(q.peek(), Some(&20));
        assert_eq!(q.length(), 1);

        q.push(30);
        assert_eq!(q.peek(), Some(&20));
        assert_eq!(q.length(), 2);

        assert_eq!(q.pop(), Some(20));
        assert_eq!(q.pop(), Some(30));
        assert_eq!(q.pop(), None);
        assert!(q.is_empty());
    }
}