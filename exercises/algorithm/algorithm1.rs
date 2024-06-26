/*
	single linked list merge
	This problem requires you to merge two ordered singly linked lists into one ordered singly linked list
*/

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

impl<T: std::cmp::PartialOrd+ Clone> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}
impl<T: std::cmp::PartialOrd+ Clone> LinkedList<T> {
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
	pub fn merge( list_a:LinkedList<T>, list_b:LinkedList<T>) -> Self
	{
	   //TODO
       /* 利用索引访问效率不高
       // 获取list的长度
       let a_len=list_a.length as i32;
       let b_len=list_b.length as i32;
	   
       let mut b_index:i32=0;
       let mut current_b_op=list_b.get(0);
       let mut result_list= LinkedList::default();
       for i in  0..a_len{
            // print!("{}",i);
            let current_a_op=list_a.get(i);
            if let Some(a)=current_a_op {//获取list_a 的项目
                while current_b_op.is_some()  {//当前 获取list_a  存在
                    let b=current_b_op.unwrap();
                    if a<=b{
                        result_list.add(a.clone());
                        break; //跳出while,a前进1个
                    }else if a>b{//b前进,进入while, 接着同current_a_op 比较
                        result_list.add(b.clone());
                        b_index=b_index+1;
                        current_b_op=list_b.get(b_index);
                    }
                }  
                if current_b_op.is_none(){//list_b 已经完成,直接添加list_a
                    result_list.add(a.clone());
                }
            }        
        }
        //合并剩余的list_b
        while b_index < b_len {
            let op_b=list_b.get(b_index);
            if let  Some(b)=op_b{
                result_list.add(b.clone());
                b_index+=1;
            }
        }
       */  
       
       
       let mut current_a_op=list_a.start;
       let mut current_b_op=list_b.start;
       let mut result_list= LinkedList::default();
       unsafe{
         while let Some(a)=current_a_op   {
             let aval=a.as_ref().val.clone();
             while let Some(b)= current_b_op {//当前 获取list_b  存在
                 let bval=b.as_ref().val.clone();
                 if aval<=bval{
                     result_list.add( aval);
                     current_a_op=a.as_ref().next;
                     break; //跳出while,a前进1个
                 }else if aval>bval{//b前进,进入while, 接着同current_a_op 比较
                     result_list.add(bval);
                     current_b_op=  b.as_ref().next;
                 }
             }  
             if current_b_op.is_none(){//list_b 已经完成,直接添加list_a
                 result_list.add(a.as_ref().val.clone());
             }
             current_a_op=a.as_ref().next;
          }
          while let  Some(b)=current_b_op{
                result_list.add(b.as_ref().val.clone());
                current_b_op=  b.as_ref().next;
          }
       }
       result_list
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