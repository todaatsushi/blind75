use crate::node::ListNode;

pub fn reverse_linked_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    // let mut temp: ListNode;
    // let mut current: ListNode;
    // let mut previous: ListNode;

    // while (head.next) {}
    ListNode::new(0);
}

#[cfg(test)]
mod tests {
    use super::ListNode;

    #[test]
    fn test_reverse_linked_list() {
        let mut current: ListNode = None;
        for n in 1..6 {
            current = ListNode::new(n, current);
            println!("{}", current.val);
        }
    }
}
