//给定一个字符串 s，找到 s 中最长的回文子串。你可以假设 s 的最大长度为 1000。 
//
// 示例 1： 
//
// 输入: "babad"
//输出: "bab"
//注意: "aba" 也是一个有效答案。
// 
//
// 示例 2： 
//
// 输入: "cbbd"
//输出: "bb"
// 
// Related Topics 字符串 动态规划 
// 👍 2719 👎 0


pub struct Solution {}

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        if s.len() == 0 {
            return "".to_string();
        }
        let mut dp = vec![vec![false; s.len()]; s.len()];
        let str = s.as_bytes();
        let mut ret = &s[0..1];
        for i in 0..s.len() - 1 {
            dp[i][i] = true;
            if str[i] == str[i + 1] {
                dp[i][i + 1] = true;
                ret = &s[i..=i + 1];
            }
        }
        for l in 3..s.len() + 1 {
            for i in 0..(s.len() - l + 1) {
                let j = i + l - 1;
                if str[i] == str[j] && dp[i + 1][j - 1] {
                    dp[i][j] = true;
                    ret = &s[i..=j];
                }
            }
        }
        String::from(ret)
    }
}
//leetcode submit region end(Prohibit modification and deletion)


#[test]
fn test_case() {
    assert_eq!(Solution::longest_palindrome(String::from("cbba")), "bb");
    assert_eq!(Solution::longest_palindrome(String::from("")),"");
    assert_eq!(Solution::longest_palindrome(String::from("aaaa")), "aaaa");
}