mod ListNode;
use ListNode::ListNode;

pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    println!("Test");
    head;
}

#[cfg(test)]
mod tests {
    use super::reverse_list;
    use ListNode::ListNode;

    #[test]
    fn test_linked_list_reverses() {
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
}
