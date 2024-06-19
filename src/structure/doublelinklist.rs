use std::fmt::Display;
use std::ptr::NonNull;

#[derive(Debug)]
struct Node<T> {
    val: T,
    nxt: Option<NonNull<Node<T>>>,
    prev: Option<NonNull<Node<T>>>,
}
impl<T> Node<T> {
    fn new(val: T) -> Self {
        Self {
            val,
            nxt: None,
            prev: None,
        }
    }
}

#[derive(Debug)]
pub struct LinkedList<T> {
    length: usize,
    head: Option<NonNull<Node<T>>>,
    tail: Option<NonNull<Node<T>>>,
}
impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            length: 0,
            head: None,
            tail: None,
        }
    }
    pub fn insert_at_head(&mut self, obj: T) {
        let mut node = Box::new(Node::new(obj));
        node.nxt = self.head;
        node.prev = None;
        let node_ptr = NonNull::new(Box::into_raw(node));
        match self.head {
            None => self.tail = node_ptr,
            Some(head_ptr) => unsafe {
                (*head_ptr.as_ptr()).prev = node_ptr;
            },
        }
        self.head = node_ptr;
        self.length += 1;
    }
    pub fn insert_at_tail(&mut self, obj: T) {
        let mut node = Box::new(Node::new(obj));
        node.prev = self.tail;
        node.nxt = None;
        let node_ptr = NonNull::new(Box::into_raw(node));
        match self.tail {
            None => self.head = node_ptr,
            Some(tail_ptr) => unsafe {
                (*tail_ptr.as_ptr()).nxt = node_ptr;
            },
        }
        self.tail = node_ptr;
        self.length += 1;
    }
    pub fn insert_at_ith(&mut self, obj: T, idx: usize) {
        if idx > self.length {
            panic!("index out of length"); //如果idx超出链表长度，panic
        }
        if idx == 0 || self.head.is_none() {
            self.insert_at_head(obj);
            return;
        }
        if idx == self.length {
            self.insert_at_tail(obj);
            return;
        }
        if let Some(mut head_ptr) = self.head {
            //找到第idx个节点
            for _ in 0..idx {
                unsafe {
                    match (*head_ptr.as_ptr()).nxt {
                        None => panic!("index out of length"),
                        Some(ptr) => head_ptr = ptr,
                    }
                }
            }
            let mut node = Box::new(Node::new(obj));
            unsafe {
                //更改node节点的nxt和prev
                node.nxt = Some(head_ptr);
                node.prev = (*head_ptr.as_ptr()).prev;
                if let Some(p) = (*head_ptr.as_ptr()).prev {
                    let node_ptr = NonNull::new(Box::into_raw(node));
                    //修改目标节点的prev节点的nxt
                    (*p.as_ptr()).nxt = node_ptr;
                    //修改目标节点的prev
                    (*head_ptr.as_ptr()).prev = node_ptr;
                }
            }
            self.length += 1;
        }
    }
    pub fn delete_head(&mut self) -> Option<T> {
        //map函数专门针对Option，内层返回NonNull<Node<T>>，map会包裹一层Option再返回
        self.head.map(|head_ptr| unsafe {
            let old_head = Box::from_raw(head_ptr.as_ptr());
            match old_head.nxt {
                //既可以用(*next_ptr.as_ptr()).prev=None，也可以用代码中的as_mut，二者的区别是
                //as_ptr无需设置为mut，但是需要解引用；
                //as_mut需要设置mut，但是直接获得可变引用，无需解引用即可访问结构体成员
                Some(mut next_ptr) => next_ptr.as_mut().prev = None,
                None => self.tail = None,
            }
            self.head = old_head.nxt;
            old_head.val
        })
    }
}

impl<T> Display for Node<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.nxt {
            None => write!(f, "{}", self.val),
            Some(node) => write!(f, "{} -> {}", self.val, unsafe { node.as_ref() }),
        }
    }
}
impl<T> Display for LinkedList<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.head {
            None => Ok(()),
            Some(node) => write!(f, "{}", unsafe { node.as_ref() }),
        }
    }
}
