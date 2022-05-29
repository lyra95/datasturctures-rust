#![allow(dead_code)]

use std::ptr::NonNull;

pub struct DoublyLinkedList<T> {
    size: usize,
    head: Option<NonNull<Node<T>>>,
    tail: Option<NonNull<Node<T>>>,
}

struct Node<T> {
    element: T,
    prev: Option<NonNull<Node<T>>>,
    next: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
    pub fn new(element: T) -> Self {
        Node {
            element,
            prev: None,
            next: None,
        }
    }
}

impl<T> DoublyLinkedList<T> {
    pub fn new() -> Self {
        DoublyLinkedList {
            size: 0,
            head: None,
            tail: None,
        }
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn pop_front(&mut self) -> Option<T> {
        if self.len() == 0 {
            return None;
        }

        let old_head = unsafe { Box::from_raw(self.head.unwrap().as_ptr()) };
        self.head = old_head.next;

        if let Some(ref mut node) = self.head {
            unsafe {
                node.as_mut().prev = None;
            }
        } else {
            self.tail = None;
        }

        self.size -= 1;
        Some(old_head.element)
    }

    pub fn pop_back(&mut self) -> Option<T> {
        if self.len() == 0 {
            return None;
        }

        let old_tail = unsafe { Box::from_raw(self.tail.unwrap().as_ptr()) };
        self.tail = old_tail.prev;

        if let Some(ref mut node) = self.tail {
            unsafe {
                node.as_mut().next = None;
            }
        } else {
            self.head = None;
        }

        self.size -= 1;
        Some(old_tail.element)
    }

    pub fn push_front(&mut self, element: T) {
        let mut new_head = Box::from(Node::new(element));
        new_head.next = self.head;

        let new_head = NonNull::new(Box::leak(new_head));

        if let Some(ref mut node) = self.head {
            unsafe {
                node.as_mut().prev = new_head;
            }
        } else {
            self.tail = new_head;
        }

        self.head = new_head;
        self.size += 1;
    }

    pub fn push_back(&mut self, element: T) {
        let mut new_tail = Box::from(Node::new(element));
        new_tail.prev = self.tail;

        let new_tail = NonNull::new(Box::leak(new_tail));

        if let Some(ref mut node) = self.tail {
            unsafe {
                node.as_mut().next = new_tail;
            }
        } else {
            self.head = new_tail;
        }

        self.tail = new_tail;
        self.size += 1;
    }

    pub fn front(&self) -> Option<&T> {
        if self.len() == 0 {
            return None;
        }

        let head = unsafe { self.head.unwrap().as_ref() };
        Some(&head.element)
    }

    pub fn back(&self) -> Option<&T> {
        if self.len() == 0 {
            return None;
        }

        let tail = unsafe { self.tail.unwrap().as_ref() };
        Some(&tail.element)
    }
}

impl<T> Drop for DoublyLinkedList<T> {
    fn drop(&mut self) {
        while self.pop_front().is_some() {}
    }
}

#[cfg(test)]
mod core_tests {
    use crate::core::DoublyLinkedList;

    #[test]
    fn test_new() {
        let l: DoublyLinkedList<i32> = DoublyLinkedList::new();
        assert_eq!(l.len(), 0);
    }

    #[test]
    fn test_push_front() {
        let mut l: DoublyLinkedList<i32> = DoublyLinkedList::new();
        l.push_front(1);
        assert_eq!(l.len(), 1);
        assert_eq!(l.front(), Some(&1));
        assert_eq!(l.back(), Some(&1));
    }

    #[test]
    fn test_push_back() {
        let mut l: DoublyLinkedList<i32> = DoublyLinkedList::new();
        l.push_back(1);
        assert_eq!(l.len(), 1);
        assert_eq!(l.front(), Some(&1));
        assert_eq!(l.back(), Some(&1));
    }

    #[test]
    fn test_pop_front() {
        let mut l: DoublyLinkedList<i32> = DoublyLinkedList::new();
        l.push_back(1);
        l.push_back(2);
        assert_eq!(l.pop_front(), Some(1));
        assert_eq!(l.len(), 1);
        assert_eq!(l.front(), Some(&2));
        assert_eq!(l.back(), Some(&2));
    }

    #[test]
    fn test_pop_back() {
        let mut l: DoublyLinkedList<i32> = DoublyLinkedList::new();
        l.push_back(1);
        l.push_back(2);
        assert_eq!(l.pop_back(), Some(2));
        assert_eq!(l.len(), 1);
        assert_eq!(l.front(), Some(&1));
        assert_eq!(l.back(), Some(&1));
    }
}
