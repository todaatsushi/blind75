mod ListNode;
mod reverse_list;

fn main() {
    use reverse_list::reverse_list;
    use ListNode::ListNode;
    println!("Solves 206 and reverses a linked list.");

    let node_2: ListNode = ListNode { val: 2, next: None };
    let node_1: ListNode = ListNode {
        val: 1,
        next: node_2,
    };
    let head: ListNode = ListNode {
        val: None,
        next: node_1,
    };

    reverse_list(head);

    let mut node: ListNode = head;
    while !Some(head.next).is_none() {
        println!("{}", node.val);
    }
}
