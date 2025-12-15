/*
	binary_search tree
	This problem requires you to implement a basic interface for a binary tree
*/

//I AM NOT DONEuse std::cmp::Ordering;
use std::fmt::Debug;


#[derive(Debug)]
struct TreeNode<T>
where
    T: Ord,
{
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

#[derive(Debug)]
struct BinarySearchTree<T>
where
    T: Ord,
{
    root: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    fn new(value: T) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }

    // 修复1：补全insert的Equal和Greater分支
    fn insert(&mut self, value: T) {
        match value.cmp(&self.value) {
            // 小于：插入左子树
            Ordering::Less => match &mut self.left {
                Some(left_node) => left_node.insert(value),
                None => self.left = Some(Box::new(TreeNode::new(value))),
            },
            // 大于：插入右子树
            Ordering::Greater => match &mut self.right {
                Some(right_node) => right_node.insert(value),
                None => self.right = Some(Box::new(TreeNode::new(value))),
            },
            // 等于：忽略重复值（BST通常不存储重复）
            Ordering::Equal => (),
        }
    }

    // 修复2：实现TreeNode的search方法
    fn search(&self, value: &T) -> bool {
        match value.cmp(&self.value) {
            // 小于：搜索左子树
            Ordering::Less => self.left.as_ref().map_or(false, |left| left.search(value)),
            // 大于：搜索右子树
            Ordering::Greater => self.right.as_ref().map_or(false, |right| right.search(value)),
            // 等于：找到目标
            Ordering::Equal => true,
        }
    }
}

impl<T> BinarySearchTree<T>
where
    T: Ord,
{
    fn new() -> Self {
        BinarySearchTree { root: None }
    }

    // 插入逻辑保持不变（已正确调用TreeNode::insert）
    fn insert(&mut self, value: T) {
        match &mut self.root {
            Some(node) => node.insert(value),
            None => self.root = Some(Box::new(TreeNode::new(value))),
        }
    }

    // 修复3：修正search逻辑，调用TreeNode::search
    fn search(&self, value: T) -> bool {
        self.root.as_ref().map_or(false, |root| root.search(&value))
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_search() {
        let mut bst = BinarySearchTree::new();

        assert_eq!(bst.search(1), false);

        bst.insert(5);
        bst.insert(3);
        bst.insert(7);
        bst.insert(2);
        bst.insert(4);

        assert_eq!(bst.search(5), true);
        assert_eq!(bst.search(3), true);
        assert_eq!(bst.search(7), true);
        assert_eq!(bst.search(2), true);
        assert_eq!(bst.search(4), true);

        assert_eq!(bst.search(1), false);
        assert_eq!(bst.search(6), false);
    }

    #[test]
    fn test_insert_duplicate() {
        let mut bst = BinarySearchTree::new();

        bst.insert(1);
        bst.insert(1);

        assert_eq!(bst.search(1), true);

        match bst.root {
            Some(ref node) => {
                assert!(node.left.is_none());
                assert!(node.right.is_none());
            },
            None => panic!("Root should not be None after insertion"),
        }
    }
}
