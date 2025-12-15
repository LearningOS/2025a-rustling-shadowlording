/*
	single linked list merge
	This problem requires you to merge two ordered singly linked lists into one ordered singly linked list
*/
// I AM NOT DONE
use std::fmt::{self, Display, Formatter};
use std::ptr::NonNull;

#[derive(Debug)]
struct Node<T> {
    val: T,
    next: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
    fn new(t: T) -> Node<T> {
        Node { val: t, next: None }
    }
}

#[derive(Debug)]
struct LinkedList<T> {
    length: u32,
    start: Option<NonNull<Node<T>>>,
    end: Option<NonNull<Node<T>>>,
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self { length: 0, start: None, end: None }
    }

    pub fn add(&mut self, obj: T) {
        let node = Box::new(Node::new(obj));
        let node_ptr = Some(unsafe { NonNull::new_unchecked(Box::into_raw(node)) });
        match self.end {
            None => self.start = node_ptr,
            Some(end_ptr) => unsafe { (*end_ptr.as_ptr()).next = node_ptr },
        }
        self.end = node_ptr;
        self.length += 1;
    }

    // 修复：get方法无需&mut self，改为&self
    pub fn get(&self, index: i32) -> Option<&T> {
        self.get_ith_node(self.start, index)
    }

    // 修复：get_ith_node改为&self
    fn get_ith_node(&self, node: Option<NonNull<Node<T>>>, index: i32) -> Option<&T> {
        match node {
            None => None,
            Some(next_ptr) => match index {
                0 => Some(unsafe { &(*next_ptr.as_ptr()).val }),
                _ => self.get_ith_node(unsafe { (*next_ptr.as_ptr()).next }, index - 1),
            },
        }
    }

    // 修复1：添加PartialOrd约束（支持<=比较）
    // 修复2：调整函数结构，将push_node移到merge外部
    pub fn merge(mut list_a: LinkedList<T>, mut list_b: LinkedList<T>) -> Self
    where
        T: PartialOrd + Clone, // 增加Clone约束，用于复制节点值（避免所有权问题）
    {
        let mut merged_list = LinkedList::new();
        let mut a_current = list_a.start;
        let mut b_current = list_b.start;

        // 遍历并合并两个有序链表
        while let (Some(a_ptr), Some(b_ptr)) = (a_current, b_current) {
            let a_val = unsafe { &(*a_ptr.as_ptr()).val };
            let b_val = unsafe { &(*b_ptr.as_ptr()).val };

            if a_val <= b_val {
                // 复制节点值，创建新节点（避免原链表所有权冲突）
                merged_list.add(a_val.clone());
                a_current = unsafe { (*a_ptr.as_ptr()).next };
            } else {
                merged_list.add(b_val.clone());
                b_current = unsafe { (*b_ptr.as_ptr()).next };
            }
        }

        // 处理list_a剩余节点
        while let Some(a_ptr) = a_current {
            merged_list.add(unsafe { (*a_ptr.as_ptr()).val.clone() });
            a_current = unsafe { (*a_ptr.as_ptr()).next };
        }

        // 处理list_b剩余节点
        while let Some(b_ptr) = b_current {
            merged_list.add(unsafe { (*b_ptr.as_ptr()).val.clone() });
            b_current = unsafe { (*b_ptr.as_ptr()).next };
        }

        merged_list
    }
}

impl<T> Display for LinkedList<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut current = self.start;
        while let Some(node_ptr) = current {
            let node = unsafe { node_ptr.as_ref() };
            write!(f, "{}", node.val)?;
            current = node.next;
            if current.is_some() {
                write!(f, ", ")?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::LinkedList;

    #[test]
    fn create_numeric_list() {
        let mut list = LinkedList::<i32>::new();
        list.add(1);
        list.add(2);
        list.add(3);
        assert_eq!(3, list.length);
    }

    #[test]
    fn create_string_list() {
        let mut list_str = LinkedList::<String>::new();
        list_str.add("A".to_string());
        list_str.add("B".to_string());
        list_str.add("C".to_string());
        assert_eq!(3, list_str.length);
    }

    #[test]
    fn test_merge_linked_list_1() {
        let mut list_a = LinkedList::<i32>::new();
        let mut list_b = LinkedList::<i32>::new();
        let vec_a = vec![1, 3, 5, 7];
        let vec_b = vec![2, 4, 6, 8];
        let target_vec = vec![1, 2, 3, 4, 5, 6, 7, 8];

        vec_a.iter().for_each(|&x| list_a.add(x));
        vec_b.iter().for_each(|&x| list_b.add(x));

        let merged_list = LinkedList::merge(list_a, list_b);
        for (i, &val) in target_vec.iter().enumerate() {
            assert_eq!(val, *merged_list.get(i as i32).unwrap());
        }
    }

    #[test]
    fn test_merge_linked_list_2() {
        let mut list_a = LinkedList::<i32>::new();
        let mut list_b = LinkedList::<i32>::new();
        let vec_a = vec![11, 33, 44, 88, 89, 90, 100];
        let vec_b = vec![1, 22, 30, 45];
        let target_vec = vec![1, 11, 22, 30, 33, 44, 45, 88, 89, 90, 100];

        vec_a.iter().for_each(|&x| list_a.add(x));
        vec_b.iter().for_each(|&x| list_b.add(x));

        let merged_list = LinkedList::merge(list_a, list_b);
        for (i, &val) in target_vec.iter().enumerate() {
            assert_eq!(val, *merged_list.get(i as i32).unwrap());
        }
    }
}
