//! Stack: LIFO wrapper over Vec<T> for parsing and DFS tasks.

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Stack<T> {
    v: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Self { v: Vec::new() }
    }
    pub fn push(&mut self, x: T) {
        self.v.push(x)
    }
    pub fn pop(&mut self) -> Option<T> {
        self.v.pop()
    }
    pub fn peek(&self) -> Option<&T> {
        self.v.last()
    }
    pub fn is_empty(&self) -> bool {
        self.v.is_empty()
    }
    pub fn len(&self) -> usize {
        self.v.len()
    }
}

#[cfg(test)]
mod tests {
    use super::Stack;
    #[test]
    fn basic() {
        let mut s = Stack::new();
        assert!(s.is_empty());
        s.push(1);
        s.push(2);
        assert_eq!(s.peek(), Some(&2));
        assert_eq!(s.pop(), Some(2));
        assert_eq!(s.pop(), Some(1));
        assert_eq!(s.pop(), None);
        assert_eq!(s.len(), 0);
    }

    #[test]
    fn peek_empty() {
        let s: Stack<i32> = Stack::new();
        assert!(s.peek().is_none());
    }
}
