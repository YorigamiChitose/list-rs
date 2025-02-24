use std::usize;

type NodePtr<T> = Option<Box<Node<T>>>;

pub struct List<T> {
    head: NodePtr<T>,
}

struct Node<T> {
    elem: T,
    next: NodePtr<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push(&mut self, elem: T) {
        let new_node = Box::new(Node {
            elem: elem,
            next: self.head.take(),
        });
        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.elem)
    }

    pub fn insert(&mut self, index: usize, elem: T) -> Result<(), &'static str> {
        let mut current = &mut self.head;

        for _ in 0..index {
            if let Some(node) = current {
                current = &mut node.next;
            } else {
                return Err("Index out of bounds");
            }
        }

        current.take().map(|node| {
            let new_node = Box::new(Node {
                elem,
                next: Some(node),
            });
            *current = Some(new_node);
        });

        Ok(())
    }

    pub fn remove(&mut self, index: usize) -> Result<(), &'static str> {
        let mut current = &mut self.head;
        for _ in 0..index {
            if let Some(node) = current {
                current = &mut node.next;
            } else {
                return Err("Index out of bounds");
            }
        }
        if let Some(node) = current.take() {
            *current = node.next;
            Ok(())
        } else {
            Err("Index out of bounds")
        }
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            next: self.head.as_deref(),
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut {
            next: self.head.as_deref_mut(),
        }
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
        }
    }
}

pub struct IntoIter<T>(List<T>);

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.elem
        })
    }
}

pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_deref_mut();
            &mut node.elem
        })
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn test_list_push() {
        let mut list_a = List::new();
        list_a.push(1);
        assert_eq!(list_a.pop(), Some(1));
    }

    #[test]
    fn test_list_pop() {
        let mut list_a = List::new();
        assert_eq!(list_a.pop(), None);
        list_a.push(1);
        list_a.push(2);
        list_a.push(3);
        assert_eq!(list_a.pop(), Some(3));
        assert_eq!(list_a.pop(), Some(2));
        list_a.push(4);
        list_a.push(5);
        assert_eq!(list_a.pop(), Some(5));
        assert_eq!(list_a.pop(), Some(4));
        assert_eq!(list_a.pop(), Some(1));
        assert_eq!(list_a.pop(), None);
    }

    #[test]
    fn test_list_peek() {
        let mut list_a = List::new();
        list_a.push(1);
        list_a.push(2);
        list_a.push(3);
        assert_eq!(list_a.pop(), Some(3));
        assert_eq!(list_a.peek(), Some(&2));
        assert_eq!(list_a.pop(), Some(2));
        assert_eq!(list_a.peek(), Some(&1));
        assert_eq!(list_a.pop(), Some(1));
        assert_eq!(list_a.peek(), None);
    }

    #[test]
    fn test_list_insert() {
        let mut list_a = List::new();
        list_a.push(1);
        list_a.push(2);
        list_a.push(3);
        let _ = list_a.insert(2, 999);

        assert_eq!(list_a.pop(), Some(3));
        assert_eq!(list_a.pop(), Some(2));
        assert_eq!(list_a.pop(), Some(999));
        assert_eq!(list_a.pop(), Some(1));
    }

    #[test]
    fn test_list_remove() {
        let mut list_a = List::new();
        list_a.push(1);
        list_a.push(2);
        list_a.push(3);
        let _ = list_a.remove(1);

        assert_eq!(list_a.pop(), Some(3));
        assert_eq!(list_a.pop(), Some(1));
    }

    #[test]
    fn test_list_into_iter() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_list_iter() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
    }

    #[test]
    fn test_list_iter_mut() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.iter_mut();
        assert_eq!(iter.next(), Some(&mut 3));
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), Some(&mut 1));
    }
}
