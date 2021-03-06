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
    /// ```
    /// use doubly_linked_list::core::DoublyLinkedList;
    /// let l: DoublyLinkedList<i32> = DoublyLinkedList::new();
    /// assert_eq!(l.len(), 0);
    /// ```
    pub fn new() -> Self {
        DoublyLinkedList {
            size: 0,
            head: None,
            tail: None,
        }
    }

    /// ```
    /// use doubly_linked_list::core::DoublyLinkedList;
    /// let mut list = DoublyLinkedList::new();
    /// assert_eq!(list.len(), 0);
    /// list.push_front(1);
    /// assert_eq!(list.len(), 1);
    /// ```
    pub fn len(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// ```
    /// use doubly_linked_list::core::DoublyLinkedList;
    /// let mut l: DoublyLinkedList<i32> = DoublyLinkedList::new();
    /// l.push_back(1);
    /// assert_eq!(l.pop_front(), Some(1));
    /// ```
    pub fn pop_front(&mut self) -> Option<T> {
        if self.is_empty() {
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

    /// ```
    /// use doubly_linked_list::core::DoublyLinkedList;
    /// let mut l: DoublyLinkedList<i32> = DoublyLinkedList::new();
    /// l.push_back(1);
    /// l.push_back(2);
    /// assert_eq!(l.pop_back(), Some(2));
    /// assert_eq!(l.len(), 1);
    /// assert_eq!(l.front(), Some(&1));
    /// assert_eq!(l.back(), Some(&1));
    /// ```
    pub fn pop_back(&mut self) -> Option<T> {
        if self.is_empty() {
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

    /// ```
    /// use doubly_linked_list::core::DoublyLinkedList;
    /// let mut l: DoublyLinkedList<i32> = DoublyLinkedList::new();
    /// l.push_back(1);
    /// l.push_back(2);
    /// l.push_back(3);
    /// assert_eq!(l.remove(1), 2);
    /// assert_eq!(l.remove(1), 3);
    /// assert_eq!(l.remove(0), 1);
    /// assert_eq!(l.len(), 0);
    /// assert_eq!(l.front(), None);
    /// assert_eq!(l.back(), None);
    /// ```
    pub fn remove(&mut self, at: usize) -> T {
        let mut at = at;

        if at >= self.len() {
            panic!("index out of bounds");
        }

        if at == 0 {
            return self.pop_front().unwrap();
        }

        if at == self.len() - 1 {
            return self.pop_back().unwrap();
        }

        let mut current = self.head.unwrap();
        while at > 0 {
            current = unsafe { current.as_ref().next.unwrap() };
            at -= 1;
        }

        let mut prev = unsafe { current.as_ref().prev.unwrap() };
        let mut next = unsafe { current.as_ref().next.unwrap() };

        unsafe {
            prev.as_mut().next = Some(next);
            next.as_mut().prev = Some(prev);
        }

        self.size -= 1;
        let poped = unsafe { Box::from_raw(current.as_ptr()) };
        poped.element
    }

    /// ```
    /// use doubly_linked_list::core::DoublyLinkedList;
    /// let mut l: DoublyLinkedList<i32> = DoublyLinkedList::new();
    /// l.push_front(1);
    /// assert_eq!(l.len(), 1);
    /// assert_eq!(l.front(), Some(&1));
    /// assert_eq!(l.back(), Some(&1));
    /// ```
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

    /// ```
    /// use doubly_linked_list::core::DoublyLinkedList;
    /// let mut l: DoublyLinkedList<i32> = DoublyLinkedList::new();
    /// l.push_back(1);
    /// assert_eq!(l.len(), 1);
    /// assert_eq!(l.front(), Some(&1));
    /// assert_eq!(l.back(), Some(&1));
    /// ```
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

    /// ```
    /// use doubly_linked_list::core::DoublyLinkedList;
    /// let mut l: DoublyLinkedList<i32> = DoublyLinkedList::from([1,2,4]);
    /// l.insert_after(1,3);
    /// assert_eq!(l.len(), 4);
    /// ```
    pub fn insert_after(&mut self, at: usize, element: T) {
        if at >= self.len() {
            panic!("Index out of bounds");
        }

        if at == self.len() - 1 {
            self.push_back(element);
            return;
        }

        let mut current = self.head.unwrap();
        let mut at = at;

        while at > 0 {
            current = unsafe { current.as_ref().next.unwrap() };
            at -= 1;
        }

        let mut node = NonNull::from(Box::leak(Box::from(Node::new(element))));
        let mut next = unsafe { current.as_ref().next.unwrap() };

        unsafe {
            next.as_mut().prev = Some(node);
            node.as_mut().next = Some(next);
            current.as_mut().next = Some(node);
            node.as_mut().prev = Some(current);
        }

        self.size += 1;
    }

    pub fn insert_before(&mut self, element: T, at: usize) {
        if at >= self.len() {
            panic!("Index out of bounds");
        }

        if at == 0 {
            self.push_front(element);
            return;
        }

        let mut at = at;
        let mut current = self.head.unwrap();
        while at > 0 {
            current = unsafe { current.as_ref().next.unwrap() };
            at -= 1;
        }

        let mut node = NonNull::from(Box::leak(Box::from(Node::new(element))));

        let mut prev = unsafe { current.as_ref().prev.unwrap() };

        unsafe {
            prev.as_mut().next = Some(node);
            node.as_mut().prev = Some(prev);
            node.as_mut().next = Some(current);
            current.as_mut().prev = Some(node);
        }

        self.size += 1;
    }

    pub fn front(&self) -> Option<&T> {
        if self.is_empty() {
            return None;
        }

        let head = unsafe { self.head.unwrap().as_ref() };
        Some(&head.element)
    }

    /// ```
    /// use doubly_linked_list::core::DoublyLinkedList;
    /// let mut l: DoublyLinkedList<i32> = DoublyLinkedList::new();
    /// l.push_back(1);
    /// l.push_back(2);
    /// l.front_mut().map(|x| *x = 3);
    /// l.back_mut().map(|x| *x = 4);
    /// assert_eq!(l.front_mut(), Some(&mut 3));
    /// assert_eq!(l.back_mut(), Some(&mut 4));
    /// ```
    pub fn front_mut(&mut self) -> Option<&mut T> {
        if self.is_empty() {
            return None;
        }

        let head = unsafe { self.head.unwrap().as_mut() };
        Some(&mut head.element)
    }

    pub fn back(&self) -> Option<&T> {
        if self.is_empty() {
            return None;
        }

        let tail = unsafe { self.tail.unwrap().as_ref() };
        Some(&tail.element)
    }

    pub fn back_mut(&mut self) -> Option<&mut T> {
        if self.is_empty() {
            return None;
        }

        let tail = unsafe { self.tail.unwrap().as_mut() };
        Some(&mut tail.element)
    }

    pub fn get(&self, at: usize) -> Option<&T> {
        if at >= self.len() {
            return None;
        }

        let mut current = self.head.unwrap();
        let mut at = at;

        while at > 0 {
            current = unsafe { current.as_ref().next.unwrap() };
            at -= 1;
        }

        Some(unsafe { &current.as_ref().element })
    }

    pub fn get_mut(&mut self, at: usize) -> Option<&mut T> {
        if at >= self.len() {
            return None;
        }

        let mut current = self.head.unwrap();
        let mut at = at;

        while at > 0 {
            current = unsafe { current.as_mut().next.unwrap() };
            at -= 1;
        }

        Some(unsafe { &mut current.as_mut().element })
    }
}

impl<T> Drop for DoublyLinkedList<T> {
    fn drop(&mut self) {
        while self.pop_front().is_some() {}
    }
}

impl<T> Default for DoublyLinkedList<T> {
    fn default() -> Self {
        DoublyLinkedList::new()
    }
}
