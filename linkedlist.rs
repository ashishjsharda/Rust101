use std::collections::linked_list::LinkedList;


fn main() {
    let mut list = LinkedList::new();
    list.push_back(1);
    list.push_back(2);
    list.push_back(3);
    println!("{:?}", list);
}
