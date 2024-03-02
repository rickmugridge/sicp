// p163 and p215
use std::cmp::Ordering;
use std::rc::Rc;

#[derive(Debug, PartialEq)]
struct BinaryTree<T> where T: Clone + Copy + PartialEq + PartialOrd {
    tree: Option<Rc<TreeNode<T>>>,
}

#[derive(Debug, PartialEq)]
struct TreeNode<T> where T: Clone + Copy + PartialEq + PartialOrd {
    value: T,
    left: Option<Rc<TreeNode<T>>>,
    right: Option<Rc<TreeNode<T>>>,
}

impl<T> BinaryTree<T>
    where T: Clone + Copy + PartialEq + PartialOrd + Ord {
    fn new(tree: Option<Rc<TreeNode<T>>>) -> Self {
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

    fn map<U: Copy>(&self, f: &dyn Fn(T) -> U) -> BinaryTree<U>
        where U: Clone + Copy + PartialEq + PartialOrd + Ord {
        match &self.tree {
            None => BinaryTree::new(None),
            Some(n) => {
                let node = n.map(f);
                let tree = Some(Rc::new(node));
                BinaryTree::new(tree)
            }
        }
    }

    fn contains(&self, x: T) -> bool {
        match &self.tree {
            None => false,
            Some(node) => node.contains(x),
        }
    }

    fn immutably_add(&self, x: T) -> Self {
        match &self.tree {
            None => Self::new(Some(Rc::new(TreeNode::new(x, None, None)))),
            Some(node) => Self::new(Some(immutably_add(node.clone(), x))),
        }
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
        BinaryTree::new(TreeNode::from_vec(&elements))
    }

    fn union(&self, other: Self) -> Self {
        let mut elements = self.to_vec();
        elements.extend_from_slice(&other.to_vec());
        elements.sort();
        elements.dedup();
        Self::from_vec(elements)
    }

    fn intersection(&self, other: &Self) -> Self {
        let mut v1 = self.to_vec();
        let mut v2 = other.to_vec();
        if v1.is_empty() || v2.is_empty() {
            return Self::new(None);
        }
        v1.sort();
        v2.sort();
        let mut pos1 = 0;
        let mut pos2 = 0;
        let mut inter = vec![];
        while pos1 < v1.len() && pos2 < v2.len() {
            match v1[pos1].cmp(&v2[pos2]) {
                Ordering::Less => { pos1 += 1; }
                Ordering::Equal => {
                    inter.push(v1[pos1]);
                    pos1 += 1;
                    pos2 += 1;
                }
                Ordering::Greater => { pos2 += 1; }
            }
        }
        Self::from_vec(inter)
    }
}

impl<T> TreeNode<T> where T: Clone + Copy + PartialEq + PartialOrd {
    fn new(value: T,
           left: Option<Rc<TreeNode<T>>>,
           right: Option<Rc<TreeNode<T>>>) -> Self {
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

    fn map<U>(&self, f: &dyn Fn(T) -> U) -> TreeNode<U> where U: Clone + Copy + PartialEq + PartialOrd {
        let new_left = self.left.as_ref().map(|b| Rc::new(b.map(&f)));
        let new_right = self.right.as_ref().map(|b| Rc::new(b.map(&f)));
        TreeNode::new(f(self.value), new_left, new_right)
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

    fn from_vec(elements: &[T]) -> Option<Rc<TreeNode<T>>> {
        if elements.is_empty() {
            return None;
        }
        let centre = elements.len() / 2;
        let left = TreeNode::from_vec(&elements[0..centre]);
        let right = TreeNode::from_vec(&elements[centre + 1..]);
        Some(Rc::new(TreeNode::new(elements[centre], left, right)))
    }
}

fn immutably_add<T>(node: Rc<TreeNode<T>>, x: T) -> Rc<TreeNode<T>> where T: Clone + Copy + PartialEq + PartialOrd {
    match x.partial_cmp(&node.value) {
        None => panic!("Cannot compare"),
        Some(Ordering::Equal) => node,
        Some(Ordering::Less) => {
            let right = cloned(&node.right);
            match &node.left {
                Some(left) => {
                    let left2 = Some(immutably_add(left.clone(), x));
                    Rc::new(TreeNode::new(node.value, left2, right))
                }
                None => {
                    let left = Some(Rc::new(TreeNode::empty(x)));
                    Rc::new(TreeNode::new(node.value, left, right))
                }
            }
        }
        Some(Ordering::Greater) => {
            let left = cloned(&node.left);
            match &node.right {
                Some(right) => {
                    let right2 = Some(immutably_add(right.clone(), x));
                    Rc::new(TreeNode::new(node.value, left, right2))
                }
                None => {
                    let right = Some(Rc::new(TreeNode::empty(x)));
                    Rc::new(TreeNode::new(node.value, left, right))
                }
            }
        }
    }
}

fn cloned<T>(node: &Option<Rc<TreeNode<T>>>) -> Option<Rc<TreeNode<T>>> where T: Clone + Copy + PartialEq + PartialOrd {
    node.as_ref().map(|r| r.clone())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count() {
        let it: BinaryTree<isize> = BinaryTree::new(Some(Rc::new(TreeNode::new(1, None, None))));
        assert_eq!(it.count(), 1);
    }

    #[test]
    fn map() {
        let it: BinaryTree<isize> = BinaryTree::new(
            Some(Rc::new(TreeNode::new(1, None, None))));
        assert_eq!(it.map(&|n: isize| n + 1), BinaryTree::new(
            Some(Rc::new(TreeNode::new(2, None, None)))));
    }

    #[test]
    fn contains() {
        let it: BinaryTree<isize> = BinaryTree::new(
            Some(Rc::new(TreeNode::new(1, None, None))));
        assert_eq!(it.contains(1), true);
        assert_eq!(it.contains(2), false);
        assert_eq!(BinaryTree::new(None).contains(1), false);
    }

    #[test]
    fn from_vec_empty() {
        let tree: BinaryTree<i32> = BinaryTree::from_vec(vec![]);
        assert_eq!(tree.to_vec(), vec![]);
    }

    #[test]
    fn from_vec() {
        let tree = BinaryTree::from_vec(vec![1, 2, 3, 4]);
        assert_eq!(tree.to_vec(), vec![1, 2, 3, 4]);
    }

    #[test]
    fn union() {
        let tree1 = BinaryTree::from_vec(vec![1, 2, 3, 4]);
        let tree2 = BinaryTree::from_vec(vec![1, 2, 3, 4]);
        assert_eq!(tree1.union(tree2).to_vec(), vec![1, 2, 3, 4])
    }

    #[test]
    fn intersection() {
        let tree1 = BinaryTree::from_vec(vec![1, 2, 3, 4]);
        let tree2 = BinaryTree::from_vec(vec![1, 2, 3, 4]);
        assert_eq!(tree1.intersection(&tree2).to_vec(), vec![1, 2, 3, 4]);

        let tree3 = BinaryTree::from_vec(vec![11, 12, 3, 14]);
        assert_eq!(tree1.intersection(&tree3).to_vec(), vec![3]);
    }
}