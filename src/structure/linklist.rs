use std::fmt::Display;
use std::ptr::NonNull;

#[derive(Debug)]
struct Node<T> {
    val: T,
    nxt: Option<NonNull<Node<T>>>,
}
impl<T> Node<T> {
    fn new(val: T) -> Self {
        Self { val, nxt: None }
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
        let node_ptr = NonNull::new(Box::into_raw(node));
        if self.head.is_none() {
            self.tail = node_ptr;
        }
        self.head = node_ptr;
        self.length += 1;
    }
    pub fn insert_at_tail(&mut self, obj: T) {
        let node_ptr = NonNull::new(Box::into_raw(Box::new(Node::new(obj))));
        match self.tail {
            None => self.head = node_ptr,
            Some(mut tail_ptr) => unsafe {
                tail_ptr.as_mut().nxt = node_ptr;
            },
        }
        self.tail = node_ptr;
        self.length += 1;
    }
    pub fn insert_at_ith(&mut self, obj: T, idx: usize) {
        if idx > self.length {
            panic!("index out of length");
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
            //找到第idx - 1个节点
            for _ in 0..idx - 1 {
                unsafe {
                    match (*head_ptr.as_ptr()).nxt {
                        None => panic!("index out of length"),
                        Some(next_node) => head_ptr = next_node,
                    }
                }
            }
            let mut node = Box::new(Node::new(obj));
            unsafe {
                node.nxt = (*head_ptr.as_ptr()).nxt;
                let node_ptr = NonNull::new(Box::into_raw(node));
                head_ptr.as_mut().nxt = node_ptr;
                self.length += 1;
            }
        }
    }
    pub fn delete_head(&mut self) {
        if self.head.is_none() {
            panic!("list is empty");
        }
        if let Some(head_ptr) = self.head {
            unsafe {
                if (*head_ptr.as_ptr()).nxt.is_none() {
                    self.tail = None;
                }
                self.head = (*head_ptr.as_ptr()).nxt;
            }
        }
    }
    pub fn delete_tail(&mut self) {
        if self.head.is_none() {
            panic!("list is empty");
        }
        if let Some(mut head_ptr) = self.head {
            unsafe {
                if (*head_ptr.as_ptr()).nxt.is_none() {
                    self.head = None;
                    self.tail = None;
                    return;
                }
                for _ in 0..self.length - 2 {
                    match (*head_ptr.as_ptr()).nxt {
                        None => panic!("out of length"),
                        Some(next_node) => head_ptr = next_node
                    }
                }
                let mut new_tail = Box::from_raw(head_ptr.as_ptr());
                new_tail.nxt = None;
                self.tail = NonNull::new(Box::into_raw(new_tail));
            }
        }
    }
    // pub fn delete_ith_node(&mut self, obj: T, idx: usize) -> Option<T> {
    //     if idx > self.length {
    //         panic!("index out of length");
    //     }
    //     if idx == 0 {
    //         return self.delete_head();
    //     }
    //     //获取第idx-1个结点

    // }
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
// #[derive(Debug)]
// struct Node<T>
// where T: Display + Clone{
//     val: T,
//     nxt: Option<Box<Node<T>>>
// }
// #[derive(Debug)]
// pub struct LinkedList<T>
// where T: Display + Clone{
//     count: usize,
//     head: Option<Box<Node<T>>>,
// }
// impl<T> Node<T>
// where T: Display + Clone{
//     fn new(val: T) -> Self {
//         Self{
//             val,
//             nxt: None
//         }
//     }
//     fn push(&mut self, val: T) {
//         match &mut self.nxt {
//             None => {
//                 let new_node = Box::new(Node::new(val));
//                 self.nxt = Some(new_node);
//             },
//             Some(v) => v.push(val)
//         }
//     }
//     fn get_ith(&mut self, idx: usize) -> &mut Self{
//         if idx == 1 {
//             return self;
//         }
//         return self.nxt.as_mut().unwrap().get_ith(idx - 1);
//     }
// }
// impl<T> LinkedList<T>
// where T: Display + Clone{
//     pub fn new() -> Self {
//         Self{
//             count: 0,
//             head: None,
//         }
//     }
//     pub fn insert_at_head(&mut self, obj: T) {
//         let mut new_node = Box::new(Node::new(obj));
//         new_node.nxt = self.head.take();
//         let node = Some(new_node);
//         self.head = node;
//         self.count += 1;
//     }
//     pub fn insert_at_tail(&mut self, obj: T) {
//         match &mut self.head {
//             None => {
//                 let new_node = Box::new(Node::new(obj));
//                 self.head = Some(new_node);
//             },
//             Some(v) => v.push(obj)
//         }
//         self.count += 1;
//     }
//     pub fn insert_at_ith(&mut self, idx: usize, obj: T) {
//         if idx > self.count || idx <= usize::MIN {
//             panic!("index out of range!");
//         }
//         let node = self.head.as_mut().unwrap().get_ith(idx);
//         let mut new_node = Box::new(Node::new(obj));
//         new_node.nxt = node.nxt.take();
//         let node_ = Some(new_node);
//         node.nxt = node_;
//         self.count += 1;
//     }
//     pub fn delete_ith_node(&mut self, idx: usize) -> Option<T> {
//         if idx > self.count || idx <= usize::MIN {
//             return None;
//         }
//         let mut res: Option<T> = None;
//         if idx == 1 {
//             res = Some(self.head.as_ref().unwrap().val.clone());
//             self.head = self.head.as_mut().unwrap().nxt.take();
//         } else{
//             let pre = self.head.as_mut().unwrap().get_ith(idx - 1);
//             res = Some(pre.nxt.as_ref().unwrap().val.clone());
//             pre.nxt = pre.nxt.as_mut().unwrap().nxt.take();
//         }
//         self.count -= 1;
//         return res;
//     }
//     pub fn get_ith_node(&mut self, idx: usize) -> Option<T> {
//         if idx > self.count || idx <= usize::MIN {
//             return None;
//         }
//         let node = self.head.as_mut().unwrap().get_ith(idx);
//         return Some(node.val.clone());
//     }
//     pub fn len(&self) -> usize {
//         self.count
//     }
// }

// impl<T> Display for Node<T>
// where T: Display + Clone{
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{} ", self.val)?;
//         match &self.nxt {
//             None => Ok(()),
//             Some(x) => write!(f, "-> {}", x)
//         }
//     }
// }

// impl<T> Display for LinkedList<T>
// where T: Display + Clone{
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match &self.head {
//             None => Ok(()),
//             Some(x) => write!(f, "{}", x)
//         }
//     }
// }
