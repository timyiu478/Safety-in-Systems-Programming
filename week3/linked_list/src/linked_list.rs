use std::fmt;
use std::option::Option;

pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
    size: usize,
}

struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    pub fn new(value: T, next: Option<Box<Node<T>>>) -> Node<T> {
        Node {value: value, next: next}
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> LinkedList<T> {
        LinkedList {head: None, size: 0}
    }
    
    pub fn get_size(&self) -> usize {
        self.size
    }
    
    pub fn is_empty(&self) -> bool {
        self.get_size() == 0
    }
    
    pub fn push_front(&mut self, value: T) {
        let new_node: Box<Node<T>> = Box::new(Node::new(value, self.head.take()));
        self.head = Some(new_node);
        self.size += 1;
    }
    
    pub fn pop_front(&mut self) -> Option<T> {
        let node: Box<Node<T>> = self.head.take()?;
        self.head = node.next; // move next node to head; the old head is dropped automatically
        self.size -= 1;
        Some(node.value)
    }
}


// trait bound to ensure T is displayable
impl<T: std::fmt::Display> fmt::Display for LinkedList<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut current: &Option<Box<Node<T>>> = &self.head;
        let mut result = String::new();
        loop {
            match current {
                Some(node) => {
                    result = format!("{} {}", result, node.value);
                    current = &node.next;
                },
                None => break,
            }
        }
        write!(f, "{}", result)
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        let mut current = self.head.take();
        while let Some(mut node) = current {
            current = node.next.take();
        }
    }
}


// trait bound to ensure T is copyable
impl<T: Copy> Clone for LinkedList<T> {
    fn clone(&self) -> Self {
        let mut new_list = LinkedList::new();
        let mut current = &self.head;
        let mut vec = Vec::new();
        while let Some(node) = current {
            current = &node.next;
            vec.push(node);
        }
        // push in reverse order to maintain the same order in the new list
        for node in vec.iter().rev() {
            new_list.push_front(node.value);
        } 
        new_list
    }
}

// trait bound to ensure T is comparable
impl<T: std::cmp::PartialEq> PartialEq for LinkedList<T> {
    fn eq(&self, other: &Self) -> bool {
        if self.size != other.size {
            return false;
        }
        let mut self_current = &self.head;
        let mut other_current = &other.head;
        for _ in 0..self.size {
            if let (Some(self_node), Some(other_node)) = (self_current, other_current) {
                if self_node.value != other_node.value {
                    return false;
                }
            }
            if let Some(node) = self_current {
                self_current = &node.next;
            }
            if let Some(node) = other_current {
                other_current = &node.next;
            }
        }
        true
    }
}
