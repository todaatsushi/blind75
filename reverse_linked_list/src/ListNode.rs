pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32, next: Option<ListNode>) -> Self {
        ListNode {
            next: next.unwrap_or(None),
            val,
        }
    }
}
