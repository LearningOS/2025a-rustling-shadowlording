/*
	single linked list merge
	This problem requires you to merge two ordered singly linked lists into one ordered singly linked list
*/
// I AM NOT DONE

use std::fmt::{self, Display, Formatter};
use std::ptr::NonNull;
use std::vec::*;

#[derive(Debug)]
struct Node<T> {
    val: T,
    next: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
    fn new(t: T) -> Node<T> {
        Node {
            val: t,
            next: None,
        }
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
        Self {
            length: 0,
            start: None,
            end: None,
        }
    }

    pub fn add(&mut self, obj: T) {
        let mut node = Box::new(Node::new(obj));
        node.next = None;
        let node_ptr = Some(unsafe { NonNull::new_unchecked(Box::into_raw(node)) });
        match self.end {
            None => self.start = node_ptr,
            Some(end_ptr) => unsafe { (*end_ptr.as_ptr()).next = node_ptr },
        }
        self.end = node_ptr;
        self.length += 1;
    }

    pub fn get(&mut self, index: i32) -> Option<&T> {
        self.get_ith_node(self.start, index)
    }

    fn get_ith_node(&mut self, node: Option<NonNull<Node<T>>>, index: i32) -> Option<&T> {
        match node {
            None => None,
            Some(next_ptr) => match index {
                0 => Some(unsafe { &(*next_ptr.as_ptr()).val }),
                _ => self.get_ith_node(unsafe { (*next_ptr.as_ptr()).next }, index - 1),
            },
        }
    }
	pub fn merge(list_a:LinkedList<T>,list_b:LinkedList<T>) -> Self
	{
		//TODO
		Self {
             let mut merged_list = LinkedList::new();
        merged_list.length = list_a.length + list_b.length; // 直接计算总长度

        // 当前待比较的节点指针（分别指向 list_a 和 list_b 的当前节点）
        let mut a_current = list_a.start;
        let mut b_current = list_b.start;

        // 遍历两个链表，比较并合并节点
        while let (Some(a_ptr), Some(b_ptr)) = (a_current, b_current) {
            // 安全解引用节点（通过 NonNull 访问原始指针）
            let a_node = unsafe { a_ptr.as_ref() };
            let b_node = unsafe { b_ptr.as_ref() };

            if a_node.val <= b_node.val {
                // 将 a_node 接入 merged_list，并移动 a_current 到下一个节点
                merged_list.push_node(a_ptr);
                a_current = a_node.next;
            } else {
                // 将 b_node 接入 merged_list，并移动 b_current 到下一个节点
                merged_list.push_node(b_ptr);
                b_current = b_node.next;
            }
        }

        // 处理 list_a 剩余节点
        while let Some(a_ptr) = a_current {
            merged_list.push_node(a_ptr);
            a_current = unsafe { a_ptr.as_ref().next };
        }

        // 处理 list_b 剩余节点
        while let Some(b_ptr) = b_current {
            merged_list.push_node(b_ptr);
            b_current = unsafe { b_ptr.as_ref().next };
        }

        merged_list
    }

    // 辅助函数：将节点指针接入 merged_list（更新 start 和 end）
    fn push_node(&mut self, node_ptr: NonNull<Node<T>>) {
        // 复制节点指针（避免原链表的 next 指针被修改）
        let mut node = unsafe { Box::from_raw(node_ptr.as_ptr()) };
        node.next = None; // 断开原链表的连接，避免循环引用
        let new_ptr = Some(unsafe { NonNull::new_unchecked(Box::into_raw(node)) });

        match self.end {
            None => self.start = new_ptr, // 新链表为空时，start 和 end 都指向新节点
            Some(end_ptr) => unsafe { (*end_ptr.as_ptr()).next = new_ptr }, // 接入尾部
        }
        self.end = new_ptr;
    }
}
        }
	


impl<T> Display for LinkedList<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.start {
            Some(node) => write!(f, "{}", unsafe { node.as_ref() }),
            None => Ok(()),
        }
    }
}

impl<T> Display for Node<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.next {
            Some(node) => write!(f, "{}, {}", self.val, unsafe { node.as_ref() }),
            None => write!(f, "{}", self.val),
        }
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
        println!("Linked List is {}", list);
        assert_eq!(3, list.length);
    }

    #[test]
    fn create_string_list() {
        let mut list_str = LinkedList::<String>::new();
        list_str.add("A".to_string());
        list_str.add("B".to_string());
        list_str.add("C".to_string());
        println!("Linked List is {}", list_str);
        assert_eq!(3, list_str.length);
    }

    #[test]
    fn test_merge_linked_list_1() {
		let mut list_a = LinkedList::<i32>::new();
		let mut list_b = LinkedList::<i32>::new();
		let vec_a = vec![1,3,5,7];
		let vec_b = vec![2,4,6,8];
		let target_vec = vec![1,2,3,4,5,6,7,8];
		
		for i in 0..vec_a.len(){
			list_a.add(vec_a[i]);
		}
		for i in 0..vec_b.len(){
			list_b.add(vec_b[i]);
		}
		println!("list a {} list b {}", list_a,list_b);
		let mut list_c = LinkedList::<i32>::merge(list_a,list_b);
		println!("merged List is {}", list_c);
		for i in 0..target_vec.len(){
			assert_eq!(target_vec[i],*list_c.get(i as i32).unwrap());
		}
	}
	#[test]
	fn test_merge_linked_list_2() {
		let mut list_a = LinkedList::<i32>::new();
		let mut list_b = LinkedList::<i32>::new();
		let vec_a = vec![11,33,44,88,89,90,100];
		let vec_b = vec![1,22,30,45];
		let target_vec = vec![1,11,22,30,33,44,45,88,89,90,100];

		for i in 0..vec_a.len(){
			list_a.add(vec_a[i]);
		}
		for i in 0..vec_b.len(){
			list_b.add(vec_b[i]);
		}
		println!("list a {} list b {}", list_a,list_b);
		let mut list_c = LinkedList::<i32>::merge(list_a,list_b);
		println!("merged List is {}", list_c);
		for i in 0..target_vec.len(){
			assert_eq!(target_vec[i],*list_c.get(i as i32).unwrap());
		}
	}
}