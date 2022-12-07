// Definition for singly-linked list.
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

struct Solution;

impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        Solution::add_two_numbers_with_carry(l1, l2, false)
            .or(Some(Box::new(ListNode::new(0))))
    }

    pub fn add_two_numbers_with_carry(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>, carry: bool) -> Option<Box<ListNode>> {
        let sum = l1.val_or_zero() + l2.val_or_zero() + if carry { 1 } else { 0 };
        if (l1 == None || l2 == None) && sum == 0 {
            return l1.or(l2);
        }
        let carry = sum >= 10;
        ListNode::of(
            sum % 10,
            Solution::add_two_numbers_with_carry(l1.next_or_none(), l2.next_or_none(), carry),
        )
    }
}

impl ListNode {
    fn of(val: i32, next: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        Some(Box::new(ListNode {
            val,
            next,
        }))
    }
}

trait ListNodeOption {
    fn val_or_zero(&self) -> i32;
    fn next_or_none(self) -> Option<Box<ListNode>>;
}

impl ListNodeOption for Option<Box<ListNode>> {

    fn val_or_zero(&self) -> i32 {
        match self {
            None => 0,
            Some(node) => node.val,
        }
    }

    fn next_or_none(self) -> Option<Box<ListNode>> {
        match self {
            None => None,
            Some(node) => node.next,
        }
    }
}
