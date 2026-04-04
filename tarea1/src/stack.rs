use super::node::Node;

pub struct Stack<T> {
    head: Option<Box<Node<T>>>,
    length: usize,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Self { 
            head: None,
            length: 0
        }
    }

    pub fn push(&mut self, value: T) {
        let new_node = Box::new(Node::new(value, self.head.take()));
        self.head = Some(new_node);
        self.length += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if let Some(node) = self.head.take() {
            self.head = node.next;
            self.length -= 1;
            return Some(node.value);
        }
        None
    }

    pub fn peek(&self) -> Option<&T> {
        if let Some(node) = &self.head {
            return Some(&node.value)
        }
        None
    }
    
    pub fn length(&self) -> usize {
        return self.length;
    }

    pub fn is_empty(&self) -> bool {
        return self.length == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let s: Stack<i32> = Stack::new();

        assert_eq!(s.length(), 0);
        assert!(s.is_empty());
    }

    #[test]
    fn test_push() {
        let mut s: Stack<i32> = Stack::new();
        s.push(2);
        s.push(3);

        assert!(!s.is_empty());
        assert!(s.length() == 2);
        assert_eq!(s.peek(), Some(&3));
    }

    #[test]
    fn test_pop() {
        let mut s: Stack<char> = Stack::new();
        s.push('a');
        s.push('z');

        assert_eq!(s.pop(), Some('z'));
        assert_eq!(s.pop(), Some('a'));
        assert_eq!(s.pop(), None);
    }

    #[test]
    fn test_peek() {
        let mut s: Stack<String> = Stack::new();
        s.push("Hello".to_string());
        s.push("World".to_string());
        let top_string = "World".to_string();

        assert_eq!(s.peek(), Some(&top_string));
        assert_eq!(s.length(), 2);
        assert_eq!(s.pop(), Some(top_string));
        s.pop();
        assert_eq!(s.peek(), None)
    }

    #[test]
    fn test_length() {
        let mut s: Stack<bool> = Stack::new();
        
        assert_eq!(s.length(), 0);
        s.push(true);
        assert_eq!(s.length(), 1);
        s.pop();
        assert_eq!(s.length(), 0);
        s.pop();
        assert_eq!(s.length(), 0);
    }

    #[test]
    fn test_is_empty() {
        let mut s: Stack<i32> = Stack::new();
        assert!(s.is_empty());
        s.push(1);
        assert!(!s.is_empty());
        s.pop();
        assert!(s.is_empty());
        s.pop();
        assert!(s.is_empty());
    }

    #[test]
    fn test_sequence() {
        let mut s: Stack<i32> = Stack::new();
        assert_eq!(s.length(), 0);

        s.push(10);
        assert_eq!(s.peek(), Some(&10));
        assert_eq!(s.length(), 1);

        s.push(20);
        assert_eq!(s.peek(), Some(&20));
        assert_eq!(s.length(), 2);

        assert_eq!(s.pop(), Some(20));
        assert_eq!(s.peek(), Some(&10));
        assert_eq!(s.length(), 1);

        s.push(30);
        assert_eq!(s.peek(), Some(&30));
        assert_eq!(s.length(), 2);

        assert_eq!(s.pop(), Some(30));
        assert_eq!(s.pop(), Some(10));
        assert_eq!(s.pop(), None);
        assert!(s.is_empty());
    }

}