/*
	queue
	This question requires you to use queues to implement the functionality of the stac



*/

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
/*
实现基于两个队列的栈 myStack，使用两个队列来模拟栈的操作。
栈（stack）是一种常用的数据结构，它遵循后进先出（LIFO）的原则。在这里，我们可以使用队列（queue）来模拟栈的功能。
队列是先进先出（FIFO）的数据结构，但我们可以通过使用两个队列来实现栈的大部分功能。
以下是如何使用队列来实现栈的功能：
入栈操作：将元素插入到一个队列中。
出栈操作：将除了待移除元素外的所有元素转移到另一个队列，然后移除那个元素，最后再将剩余元素转回原来的队列。
查看栈顶元素：查看另一个队列的队首元素。
判断栈是否为空：判断其中一个队列是否为空。

https://blog.csdn.net/m0_63020222/article/details/123735378
https://blog.csdn.net/TUT0UB0Y/article/details/124184921


与使用一个队列实现栈相比，使用两个队列实现栈的好处包括：

操作效率： 使用两个队列实现栈可以避免在每次出栈操作时都需要反复调整队列的元素顺序，因此在某些情况下，它可能比使用一个队列实现栈具有更好的操作效率。

并发性*： 使用两个队列实现栈可以更好地支持并发操作，因为它避免了在入栈和出栈操作之间的竞争条件，从而降低了并发操作的复杂性。


用q1模拟入栈，q2模拟出栈
*/
pub struct myStack<T>
{
	//TODO
	q1:Queue<T>,
	q2:Queue<T>
}
impl<T> myStack<T> {
    pub fn new() -> Self {
        Self {
			//TODO
			q1:Queue::<T>::new(),
			q2:Queue::<T>::new()
        }
    }
    //进行入栈操作，则只需要将元素入队到不为空的队列中即可
    //将元素入队到不为空的队列当中（若初始两个队列都为空，则只需任意入队到一个队列中即可）
    pub fn push(&mut self, elem: T) {
        //TODO
        if self.q1.is_empty() {//q1 为空 
            self.q2.enqueue(elem);//插入到q2中
        } else {
            self.q1.enqueue(elem);
        }
    }
   
    pub fn pop(&mut self) -> Result<T, &str> {
        //TODO
		// Err("Stack is empty")
        if self.is_empty() {
            return Err("Stack is empty");
        }
        // 获取非空队列
        let (non_empty_queue, empty_queue) = if self.q1.is_empty() {
            (&mut self.q2, &mut self.q1)
        } else {
            (&mut self.q1, &mut self.q2)
        };

        while non_empty_queue.size() > 1 {
             //将不为空的队列的前n-1一个元素依次出队
            if let Ok(elem) = non_empty_queue.dequeue() {
                //再入队到另一个空队列中，最后一个元素只出队不入队即可。（前提：栈不为空）
                empty_queue.enqueue(elem);
            }
        }
        //最后一个元素只出队不入队即可
        non_empty_queue.dequeue().map_err(|_| "Stack is empty")
    }
    //对于栈的判空和判满操作，只需判断栈内部两个队列的空满情况即可，要注意的是，队列一个为满即栈满，队列两个都为空，栈为空。
    pub fn is_empty(&self) -> bool {
		//TODO
        self.q1.is_empty() && self.q2.is_empty()
    }
}

#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn test_queue(){
		let mut s = myStack::<i32>::new();
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
}