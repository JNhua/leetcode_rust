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


pub struct Solution{}

//leetcode submit region begin(Prohibit modification and deletion)
// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

fn get_num(mut l: Option<Box<ListNode>>) -> i32{
    let mut n = 0;
    while l.is_some() {
        n = n*10+ l.clone().unwrap().val;
        l = l.unwrap().next;
    }
    n
}

fn get_list(mut n: i32) -> Box<ListNode>{
    let mut v = Vec::new();
    if n==0{
        v.push(n);
    } else{
        while n!=0 {
            v.push(n%10);
            n/=10;
        }
    }

    let mut head = Box::new(ListNode::new(v[0]));
    let mut last = &mut head;
    let mut first = true;
    for num in v.iter(){
        if first{
            first = false;
            continue;
        }
        let mut node = Box::new(ListNode::new(*num));
        last.next = Some(node);
        last = &mut node;
    }
    head
}

impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let n1 = get_num(l1.clone());
        let n2 = get_num(l2.clone());
        let ans = n1 + n2;
        Some(get_list(ans))
    }
}
//leetcode submit region end(Prohibit modification and deletion)
