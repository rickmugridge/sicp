// p163 and p215

// todo start of edit

use std::cmp::Ordering;
use std::fmt::Debug;

#[derive(Debug, PartialEq)]
struct MutableBinaryTree<T> where T: Clone + Copy + PartialEq + PartialOrd {
    tree: Option<Box<MutableTreeNode<T>>>,
}

#[derive(Debug, PartialEq)]
struct MutableTreeNode<T> where T: Clone + Copy + PartialEq + PartialOrd {
    value: T,
    left: Option<Box<MutableTreeNode<T>>>,
    right: Option<Box<MutableTreeNode<T>>>,
}

impl<T> MutableBinaryTree<T> where T: Clone + Copy + PartialEq + PartialOrd {
    fn new(tree: Option<Box<MutableTreeNode<T>>>) -> Self {
        Self { tree }
    }

    fn count(&self) -> isize {
        match &self.tree {
            None => 0,
            Some(n) => {
                n.count()
            }
        }
    }

    fn map<U: Copy>(&self, f: &dyn Fn(T) -> U) -> MutableBinaryTree<U> where U: Clone + Copy + PartialEq + PartialOrd {
        match &self.tree {
            None => MutableBinaryTree::new(None),
            Some(n) => {
                let node = n.map(f);
                let tree = Some(Box::new(node));
                MutableBinaryTree::new(tree)
            }
        }
    }

    fn contains(&self, x: T) -> bool {
        match &self.tree {
            None => false,
            Some(node) => node.contains(x),
        }
    }

    fn insert(&mut self, x: T) -> bool {
        let mut current_node = &mut self.tree;
        while let Some(node) = current_node {
            match &x.partial_cmp(&node.value) {
                None => panic!("Cannot compare"),
                Some(Ordering::Less) => { current_node = &mut node.left; }
                Some(Ordering::Equal) => { return false; }
                Some(Ordering::Greater) => { current_node = &mut node.right; }
            }
        }
        *current_node = Some(Box::new(MutableTreeNode::new(x, None, None)));
        true
    }

    fn to_vec(&self) -> Vec<T> {
        match &self.tree {
            None => vec![],
            Some(node) => {
                let mut elements: Vec<T> = vec![];
                node.to_vec(&mut elements);
                elements
            }
        }
    }

    fn from_vec(elements: Vec<T>) -> Self {
        MutableBinaryTree::new(MutableTreeNode::from_vec(&elements))
    }

    fn find_by_key<K>(&self, key: K) -> Option<T>
        where T: Record<K> + Copy + Debug,
              K: Copy + Debug {
        match &self.tree {
            None => None,
            Some(tree) => tree.find_by_key(key)
        }
    }
}

trait Record<K> {
    fn compare(&self, key: &K) -> Ordering;
}

impl<T> MutableTreeNode<T> where T: Clone + Copy + PartialEq + PartialOrd {
    fn new(value: T,
           left: Option<Box<MutableTreeNode<T>>>,
           right: Option<Box<MutableTreeNode<T>>>) -> Self {
        Self { value, left, right }
    }

    fn empty(value: T) -> Self {
        Self::new(value, None, None)
    }

    fn count(&self) -> isize {
        let left_count = if let Some(left) = &self.left {
            left.count()
        } else {
            0
        };
        let right_count = if let Some(right) = &self.right {
            right.count()
        } else {
            0
        };
        1 + left_count + right_count
    }

    fn map<U>(&self, f: &dyn Fn(T) -> U) -> MutableTreeNode<U> where U: Clone + Copy + PartialEq + PartialOrd {
        let new_left = self.left.as_ref().map(|b| Box::new(b.map(&f)));
        let new_right = self.right.as_ref().map(|b| Box::new(b.map(&f)));
        MutableTreeNode::new(f(self.value), new_left, new_right)
    }

    fn contains(&self, x: T) -> bool {
        if x == self.value {
            true
        } else if x < self.value {
            match &self.left {
                Some(left) => left.contains(x),
                None => false
            }
        } else {
            match &self.right {
                Some(right) => right.contains(x),
                None => false
            }
        }
    }

    fn to_vec(&self, elements: &mut Vec<T>) {
        if let Some(left) = &self.left {
            left.to_vec(elements);
        }
        elements.push(self.value);
        if let Some(right) = &self.right {
            right.to_vec(elements);
        }
    }

    fn from_vec(elements: &[T]) -> Option<Box<MutableTreeNode<T>>> {
        if elements.is_empty() {
            return None;
        }
        let centre = elements.len() / 2;
        let left = MutableTreeNode::from_vec(&elements[0..centre]);
        let right = MutableTreeNode::from_vec(&elements[centre + 1..]);
        Some(Box::new(MutableTreeNode::new(elements[centre], left, right)))
    }

    fn find_by_key<K>(&self, key: K) -> Option<T> where
        T: Record<K> + Copy + Debug,
        K: Copy + Debug {
        match self.value.compare(&key) {
            Ordering::Less => match &self.left {
                None => None,
                Some(left) => left.find_by_key(key),
            }
            Ordering::Equal => Some(self.value),
            Ordering::Greater => match &self.right {
                None => None,
                Some(right) => right.find_by_key(key),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count() {
        let it: MutableBinaryTree<isize> = MutableBinaryTree::new(Some(Box::new(MutableTreeNode::new(1, None, None))));
        assert_eq!(it.count(), 1);
    }

    #[test]
    fn map() {
        let it: MutableBinaryTree<isize> = MutableBinaryTree::new(
            Some(Box::new(MutableTreeNode::new(1, None, None))));
        assert_eq!(it.map(&|n: isize| n + 1), MutableBinaryTree::new(
            Some(Box::new(MutableTreeNode::new(2, None, None)))));
    }

    #[test]
    fn contains() {
        let it: MutableBinaryTree<isize> = MutableBinaryTree::new(
            Some(Box::new(MutableTreeNode::new(1, None, None))));
        assert_eq!(it.contains(1), true);
        assert_eq!(it.contains(2), false);
        assert_eq!(MutableBinaryTree::new(None).contains(1), false);
    }

    #[test]
    fn insert() {
        let mut tree = MutableBinaryTree::new(None);
        tree.insert(4);
        tree.insert(2);
        tree.insert(3);
        tree.insert(1);
        assert_eq!(tree.to_vec(), vec![1, 2, 3, 4]);
    }

    #[test]
    fn from_vec_empty() {
        let tree: MutableBinaryTree<i32> = MutableBinaryTree::from_vec(vec![]);
        assert_eq!(tree.to_vec(), vec![]);
    }

    #[test]
    fn from_vec() {
        let tree = MutableBinaryTree::from_vec(vec![1, 2, 3, 4]);
        assert_eq!(tree.to_vec(), vec![1, 2, 3, 4]);
    }

    #[test]
    fn info_retrieval() {
        #[derive(Debug, PartialEq, PartialOrd)]
        struct Person<'a> {
            id: usize,
            name: &'a str,
        }
        impl<'a> Person<'a> {
            fn new(id: usize, name: &'a str) -> Self {
                Self { id, name }
            }
        }
        impl<'a> Record<usize> for Person<'a> {
            fn compare(&self, key: &usize) -> Ordering {
                key.cmp(&self.id)
            }
        }

        impl<'a> Clone for Person<'a> {
            fn clone(&self) -> Self {
                Self::new(self.id, self.name.clone())
            }
        }
        impl<'a> Copy for Person<'a> {}
        let apple = Person::new(1, "Apple");
        let pie = Person::new(2, "Pie");
        let tree = MutableBinaryTree::from_vec(
            vec![apple, pie]);
        assert_eq!(tree.find_by_key(1), Some(apple));
        assert_eq!(tree.find_by_key(2), Some(pie));
        assert_eq!(tree.find_by_key(3), None);
    }
}