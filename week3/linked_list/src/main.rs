use linked_list::LinkedList;
pub mod linked_list;

fn main() {
    let mut list: LinkedList<u32> = LinkedList::new();
    assert!(list.is_empty());
    assert_eq!(list.get_size(), 0);
    for i in 1..12 {
        list.push_front(i);
    }
    println!("{}", list);
    println!("list size: {}", list.get_size());
    println!("top element: {}", list.pop_front().unwrap());
    println!("{}", list);
    println!("size: {}", list.get_size());
    println!("{}", list.to_string()); // ToString impl for anything impl Display


    let mut list2: LinkedList<String> = LinkedList::new();
    assert!(list2.is_empty());
    assert_eq!(list2.get_size(), 0);
    for i in 1..12 {
        list2.push_front(i.to_string());
    }
    println!("{}", list2);
    println!("list2 size: {}", list.get_size());
    println!("top element: {}", list2.pop_front().unwrap());
    println!("{}", list2);
    println!("size: {}", list2.get_size());
    println!("{}", list2.to_string()); // ToString impl for anything impl Display

    // Test Clone and PartialEq
    let mut list_clone: LinkedList<u32> = list.clone();
    assert_eq!(list.get_size(), list_clone.get_size());
    assert!(list == list_clone);
    list_clone.pop_front();
    assert!(list != list_clone);
   
    // If you implement iterator trait:
    //for val in &list {
    //    println!("{}", val);
    //}
}
