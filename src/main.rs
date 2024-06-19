mod structure;
// use structure::doublelinklist::LinkedList;
use structure::linklist::LinkedList;
fn main() {
    let mut list1 = LinkedList::new();
    list1.insert_at_tail(1);
    // list1.insert_at_tail(2);
    // list1.insert_at_tail(3);
    // list1.insert_at_tail(4);
    println!("{}", list1);
    list1.delete_tail();
    println!("{}", list1);
}
