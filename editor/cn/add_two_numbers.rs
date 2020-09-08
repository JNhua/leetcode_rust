//给出两个 非空 的链表用来表示两个非负的整数。其中，它们各自的位数是按照 逆序 的方式存储的，并且它们的每个节点只能存储 一位 数字。 
//
// 如果，我们将这两个数相加起来，则会返回一个新的链表来表示它们的和。 
//
// 您可以假设除了数字 0 之外，这两个数都不会以 0 开头。 
//
// 示例： 
//
// 输入：(2 -> 4 -> 3) + (5 -> 6 -> 4)
//输出：7 -> 0 -> 8
//原因：342 + 465 = 807
// 
// Related Topics 链表 数学 
// 👍 4847 👎 0


pub struct Solution {}

//leetcode submit region begin(Prohibit modification and deletion)
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

fn insert_back(mut tail: &mut Box<ListNode>, val: i32) -> &mut Box<ListNode>{
    let new_node = Box::new(ListNode::new(val));
    tail.next = Some(new_node);
    tail = tail.next.as_mut().unwrap();
    tail
}

fn add_two_node(num: &mut i32, carry: &mut i32) {
    if *num >= 10 {
        *num -= 10;
        *carry = 1;
    } else {
        *carry = 0;
    }
}

impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = Box::new(ListNode::new(0));
        let mut tail = &mut head;
        let mut carry = 0;
        let (mut p1,mut p2) = (l1.as_ref(),l2.as_ref());
        while let Some(node1) = p1.as_ref() {
            let mut num = 0;
            if let Some(node2) = p2.as_ref() {
                num = &node1.val + &node2.val + carry;
                p2 = node2.next.as_ref();
            } else {
                num = node1.val + carry;
            }
            add_two_node(&mut num, &mut carry);
            tail = insert_back(tail, num);
            p1 = node1.next.as_ref();
        }
        while let Some(node2) = p2.as_ref() {
            let mut num = &node2.val + carry;
            add_two_node(&mut num,&mut carry);
            tail = insert_back(tail, num);
            p2 = node2.next.as_ref();
        }
        if carry != 0{
            tail = insert_back(tail, carry);
        }
        head.next
    }
}
//leetcode submit region end(Prohibit modification and deletion)

#[test]
fn test_case() {
    fn get_list(mut n: i32) -> Option<Box<ListNode>> {
        let mut v = Vec::new();
        if n == 0 {
            v.push(n);
        } else {
            while n != 0 {
                v.push(n % 10);
                n /= 10;
            }
        }

        let mut head = Box::new(ListNode::new(0));
        let mut tail = &mut head;
        for num in v.iter() {
            let new_node = Box::new(ListNode::new(*num));
            tail.next = Some(new_node);
            tail = tail.next.as_mut().unwrap();
        }
        head.next
    }

    let l1 = get_list(342);
    let l2 = get_list(465);
    let ans = Solution::add_two_numbers(l1, l2);
    let mut tail = ans.as_ref();
    while let Some(node) = tail{
        println!("{}", node.val);
        tail = node.next.as_ref();
    }
}