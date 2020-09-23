//给定一个字符串，请你找出其中不含有重复字符的 最长子串 的长度。 
//
// 示例 1: 
//
// 输入: "abcabcbb"
//输出: 3 
//解释: 因为无重复字符的最长子串是 "abc"，所以其长度为 3。
// 
//
// 示例 2: 
//
// 输入: "bbbbb"
//输出: 1
//解释: 因为无重复字符的最长子串是 "b"，所以其长度为 1。
// 
//
// 示例 3: 
//
// 输入: "pwwkew"
//输出: 3
//解释: 因为无重复字符的最长子串是 "wke"，所以其长度为 3。
//     请注意，你的答案必须是 子串 的长度，"pwke" 是一个子序列，不是子串。
// 
// Related Topics 哈希表 双指针 字符串 Sliding Window 
// 👍 4359 👎 0


pub struct Solution {}

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut dp = Vec::with_capacity(s.len());
        let mut max_len: i32 = 0;
        let str = s.as_bytes();
        let mut repeat;
        let mut pos = 0usize;

        for i in 0..str.len() {
            repeat = false;
            if i == 0 {
                dp.push(1);
                pos = i;
                max_len = 1;
            } else {
                for j in pos..i {
                    if str[j] == str[i] {
                        repeat = true;
                        pos = j + 1;
                        break;
                    }
                }
                if !repeat {
                    dp.push(dp[i - 1] + 1);
                    if dp[i] > max_len {
                        max_len = dp[i];
                    }
                } else {
                    dp.push((i - pos + 1) as i32);
                }
            }
        }
        max_len
    }
}
//leetcode submit region end(Prohibit modification and deletion)

#[test]
fn test_case() {
    assert_eq!(Solution::length_of_longest_substring(String::from("abcabcbb")), 3);
}