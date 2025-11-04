/*
	queue
	This question requires you to use queues to implement the functionality of the stac
*/
// I AM NOT DONE

#[derive(Debug)]
pub struct Queue<T> {
    elements: Vec<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }

    pub fn enqueue(&mut self, value: T) {
        self.elements.push(value)
    }

    pub fn dequeue(&mut self) -> Result<T, &str> {
        if !self.elements.is_empty() {
            Ok(self.elements.remove(0usize))
        } else {
            Err("Queue is empty")
        }
    }

    pub fn peek(&self) -> Result<&T, &str> {
        match self.elements.first() {
            Some(value) => Ok(value),
            None => Err("Queue is empty"),
        }
    }

    pub fn size(&self) -> usize {
        self.elements.len()
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
}

impl<T> Default for Queue<T> {
    fn default() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }
}

pub struct MyStack<T> {
    q1: Queue<T>,
    q2: Queue<T>,
}

impl<T> MyStack<T> {
    pub fn new() -> Self {
        Self {
            q1: Queue::<T>::new(),
            q2: Queue::<T>::new(),
        }
    }
    
    pub fn push(&mut self, elem: T) {
        // 总是推入非空队列（如果都为空，推入q1）
        if !self.q1.is_empty() {
            self.q1.enqueue(elem);
        } else {
            self.q2.enqueue(elem);
        }
    }
    
    pub fn pop(&mut self) -> Result<T, &str> {
        if self.is_empty() {
            return Err("Stack is empty");
        }
        
        // 确定哪个队列有数据
        let (source, target) = if !self.q1.is_empty() {
            (&mut self.q1, &mut self.q2)
        } else {
            (&mut self.q2, &mut self.q1)
        };
        
        // 将源队列中的元素（除了最后一个）全部转移到目标队列
        while source.size() > 1 {
            if let Ok(element) = source.dequeue() {
                target.enqueue(element);
            }
        }
        
        // 返回源队列的最后一个元素（即栈顶元素）
        source.dequeue()
    }
    
    pub fn is_empty(&self) -> bool {
        self.q1.is_empty() && self.q2.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_stack() {
        let mut s = MyStack::<i32>::new();
        assert_eq!(s.pop(), Err("Stack is empty"));
        s.push(1);
        s.push(2);
        s.push(3);
        assert_eq!(s.pop(), Ok(3));
        assert_eq!(s.pop(), Ok(2));
        s.push(4);
        s.push(5);
        assert_eq!(s.is_empty(), false);
        assert_eq!(s.pop(), Ok(5));
        assert_eq!(s.pop(), Ok(4));
        assert_eq!(s.pop(), Ok(1));
        assert_eq!(s.pop(), Err("Stack is empty"));
        assert_eq!(s.is_empty(), true);
    }
    
    #[test]
    fn test_stack_string() {
        let mut s = MyStack::<String>::new();
        s.push("hello".to_string());
        s.push("world".to_string());
        assert_eq!(s.pop(), Ok("world".to_string()));
        assert_eq!(s.pop(), Ok("hello".to_string()));
    }
    
    #[test]
    fn test_stack_empty() {
        let mut s = MyStack::<i32>::new();
        assert!(s.is_empty());
        s.push(42);
        assert!(!s.is_empty());
        assert_eq!(s.pop(), Ok(42));
        assert!(s.is_empty());
    }
}

fn main() {
    println!("测试基于队列的栈实现...");
    
    let mut stack = MyStack::new();
    
    // 测试基本操作
    stack.push(1);
    stack.push(2);
    stack.push(3);
    
    println!("弹出: {:?}", stack.pop()); // 应该输出 3
    println!("弹出: {:?}", stack.pop()); // 应该输出 2
    
    stack.push(4);
    println!("弹出: {:?}", stack.pop()); // 应该输出 4
    println!("弹出: {:?}", stack.pop()); // 应该输出 1
    println!("弹出: {:?}", stack.pop()); // 应该输出错误
    
    println!("所有测试完成！");
}