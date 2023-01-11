#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val,
        }
    }
}


pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
    if head.is_none() {
        return false;
    }
    let mut straight = head.clone();
    let mut reversed = build_reverse(&mut head.clone());
    while let Some(mut straight_head) = straight.take() {
        let straight_next = straight_head.next.take();
        let reversed_inner = match reversed {
            Some(value) => value,
            None => break,
        };
        if straight_head.val != reversed_inner.val {
            return false;
        }
        straight = straight_next;
        reversed = reversed_inner.next;
    }
    return true;
}

pub fn build_reverse(head: &mut Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut current_node = head.take();
    let mut prev = None;
    while let Some(mut current_node_inner) = current_node.take() {
        let next = current_node_inner.next.take();
        current_node_inner.next = prev.take();
        prev = Some(current_node_inner);
        current_node = next;
    }
    return prev.take();
}
