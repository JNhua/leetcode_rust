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
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//     pub val: i32,
//     pub next: Option<Box<ListNode>>,
// }
//
// impl ListNode {
//     #[inline]
//     fn new(val: i32) -> Self {
//         ListNode {
//             next: None,
//             val,
//         }
//     }
// }

struct List {
    head: Option<Box<ListNode>>,
}

impl List {
    fn push(&mut self, val: i32) {
        let new_node = Box::new(ListNode {
            val,
            next: self.head.take(),
        });
        self.head = Some(new_node);
    }
    fn pop(&mut self) -> Option<i32> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.val
        })
    }

    fn into_iter(self) -> IntoIter {
        IntoIter(self)
    }
}

struct IntoIter(List);

impl Iterator for IntoIter {
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut ans = List { head: None };
        let mut ans_vec = Vec::new();
        let mut carry = 0;
        let mut iter1 = List { head: l1 }.into_iter();
        let mut iter2 = List { head: l2 }.into_iter();
        while let Some(val1) = iter1.next() {
            let mut num = 0;
            if let Some(val2) = iter2.next() {
                num = val1 + val2 + carry;
            } else {
                num = val1 + carry;
            }
            if num >= 10 {
                num -= 10;
                carry = 1;
            } else {
                carry = 0;
            }
            ans_vec.push(num);
        }
        while let Some(val2) = iter2.next() {
            let mut num = val2 + carry;
            if num >= 10 {
                num -= 10;
                carry = 1;
            } else {
                carry = 0;
            }
            ans_vec.push(num);
        }
        if carry != 0{
            ans_vec.push(carry);
        }
        ans_vec.reverse();
        for num in ans_vec.iter() {
            ans.push(*num);
        }
        ans.head
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

        let mut list = List { head: None };
        v.reverse();
        for num in v.iter() {
            list.push(*num);
        }
        list.head
    }

    let l1 = get_list(342);
    let l2 = get_list(465);
    let ans = Solution::add_two_numbers(l1, l2);
    let mut ans_iter = List { head: ans }.into_iter();
    assert_eq!(ans_iter.next(), Some(7));
    assert_eq!(ans_iter.next(), Some(0));
    assert_eq!(ans_iter.next(), Some(8));
}