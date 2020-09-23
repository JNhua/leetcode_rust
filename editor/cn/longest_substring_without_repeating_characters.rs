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
        let mut max_len = 0;
        let mut sub_str = vec![];
        let mut repeat = false;
        for (i, c) in s.chars().enumerate() {
            repeat = false;
            if i == 0 {
                dp.push(1);
                sub_str.push(c);
            } else {
                let mut split_index: usize = 0;
                for (j, str) in sub_str {
                    if str == c {
                        repeat = true;
                        split_index = j;
                        break;
                    }
                }
                if !repeat {
                    println!("{}, {}", pre, c);
                    dp.push(dp[i - 1] + 1);
                    if dp[i] > max_len {
                        max_len = dp[i];
                    }
                } else {
                    for j in 0..split_index + 1 {
                        sub_str.remove(j);
                    }
                }
                sub_str.push(c);
            }
        }
        max_len
    }
//leetcode submit region end(Prohibit modification and deletion)

    #[test]
    fn test_case() {
        assert_eq!(Solution::length_of_longest_substring(String::from("abcabcbb")), 3);
    }