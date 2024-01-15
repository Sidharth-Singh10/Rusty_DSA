#[derive(Debug)]
struct ListNode{
    val: Option<u32>,
    next: Option<Box<ListNode>>,
}


fn main() {
    let mut head = Box::new(ListNode { val:None,  next: None });
    let mut temp= &mut head;

    for _i in 1..10
    {
        let new_node = Box::new(ListNode { val:None,next: None });
        temp.next = Some(new_node);
        temp = temp.next.as_mut().unwrap();
    }

    
    let mut current = &head;

    while let Some(node) = current.next.as_deref()
    {
        match &node.next
        {
            Some(address) => println!("address: {:p}", address),
            None => println!("None"),
        }
        println!();
        current = current.next.as_ref().unwrap();
    }


}
