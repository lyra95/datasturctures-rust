use crate::core::DoublyLinkedList;

pub struct IntoIter<T> {
    list: DoublyLinkedList<T>,
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    /// ```
    /// use doubly_linked_list::core::DoublyLinkedList;
    /// let mut list = DoublyLinkedList::new();
    /// list.push_front(1);
    /// list.push_front(2);
    /// list.push_front(3);
    /// let mut iter = list.into_iter();
    /// assert_eq!(iter.next(), Some(3));
    /// assert_eq!(iter.next_back(), Some(1));
    /// assert_eq!(iter.next(), Some(2));
    /// assert_eq!(iter.next_back(), None);
    /// ```
    fn next(&mut self) -> Option<Self::Item> {
        self.list.pop_front()
    }
}

impl<T> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.list.pop_back()
    }
}

impl<T> IntoIterator for DoublyLinkedList<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    /// ```
    /// use doubly_linked_list::core::DoublyLinkedList;
    /// let mut list = DoublyLinkedList::new();
    /// list.push_back(1);
    /// list.push_back(2);
    /// list.push_back(3);
    /// let mut i = 1;
    /// for e in list {
    ///     assert_eq!(e, i);
    ///     i += 1;
    /// }
    /// ```
    fn into_iter(self) -> Self::IntoIter {
        IntoIter { list: self }
    }
}

impl<T> FromIterator<T> for DoublyLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut list = DoublyLinkedList::new();
        for i in iter {
            list.push_back(i);
        }
        list
    }
}

/// ```
/// ```
impl<T, const N: usize> From<[T; N]> for DoublyLinkedList<T> {
    fn from(array: [T; N]) -> Self {
        DoublyLinkedList::from_iter(array)
    }
}
