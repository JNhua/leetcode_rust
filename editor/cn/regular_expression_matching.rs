//给你一个字符串 s 和一个字符规律 p，请你来实现一个支持 '.' 和 '*' 的正则表达式匹配。 
//
// '.' 匹配任意单个字符
//'*' 匹配零个或多个前面的那一个元素
// 
//
// 所谓匹配，是要涵盖 整个 字符串 s的，而不是部分字符串。 
//
// 说明: 
//
// 
// s 可能为空，且只包含从 a-z 的小写字母。 
// p 可能为空，且只包含从 a-z 的小写字母，以及字符 . 和 *。 
// 
//
// 示例 1: 
//
// 输入:
//s = "aa"
//p = "a"
//输出: false
//解释: "a" 无法匹配 "aa" 整个字符串。
// 
//

// 示例 2: 
//
// 输入:
//s = "aa"
//p = "a*"
//输出: true
//解释: 因为 '*' 代表可以匹配零个或多个前面的那一个元素, 在这里前面的元素就是 'a'。因此，字符串 "aa" 可被视为 'a' 重复了一次。
// 
//
// 示例 3: 
//
// 输入:
//s = "ab"
//p = ".*"
//输出: true
//解释: ".*" 表示可匹配零个或多个（'*'）任意字符（'.'）。
// 
//
// 示例 4: 
//
// 输入:
//s = "aab"
//p = "c*a*b"
//输出: true
//解释: 因为 '*' 表示零个或多个，这里 'c' 为 0 个, 'a' 被重复一次。因此可以匹配字符串 "aab"。
// 
//
// 示例 5: 
//
// 输入:
//s = "mississippi"
//p = "mis*is*p*."
//输出: false 
// Related Topics 字符串 动态规划 回溯算法 
// 👍 1570 👎 0


pub struct Solution {}

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    // // 递归：执行耗时:176 ms,击败了12.07% 的Rust用户 内存消耗:1.9 MB,击败了86.67%
    // pub fn is_match(s: String, p: String) -> bool {
    //     if p.is_empty() {
    //         return s.is_empty();
    //     }
    //     let first_match = (!s.is_empty() && (p.chars().nth(0).unwrap() == s.chars().nth(0).unwrap() || p.chars().nth(0).unwrap() == '.'));
    //     if p.len() >= 2 && p.chars().nth(1).unwrap() == '*' {
    //         Solution::is_match(String::from(&s), String::from(&p[2..])) || (first_match && Solution::is_match(String::from(&s[1..]), p))
    //     } else {
    //         first_match && Solution::is_match(String::from(&s[1..]), String::from(&p[1..]))
    //     }
    // }

    // 0 ms,击败了100.00% 的Rust用户 内存消耗:1.9 MB,击败了96.67%
    pub fn is_match(s: String, p: String) -> bool {
        let p_chars: Vec<char> = p.chars().collect();

        let s_chars: Vec<char> = s.chars().collect();
        let mut dp: Vec<Vec<bool>> = vec![vec![false; p_chars.len() + 1]; s_chars.len() + 1];
        dp[0][0] = true;
        for i in 0..p_chars.len() {
            if p_chars[i] == '*' {
                dp[0][i + 1] = dp[0][i - 1];
            } else {
                dp[0][i + 1] = false;
            }
        }
        for i in 1..p_chars.len() + 1 {
            for j in 1..s_chars.len() + 1 {
                if p_chars[i - 1] == s_chars[j - 1] || p_chars[i - 1] == '.' {
                    dp[j][i] = dp[j - 1][i - 1];
                } else {
                    if p_chars[i - 1] == '*' {
                        if p_chars[i - 2] == s_chars[j - 1] || p_chars[i - 2] == '.' {
                            dp[j][i] = dp[j - 1][i] || dp[j][i - 2];
                        } else {
                            dp[j][i] = dp[j][i - 2];
                        }
                    } else {
                        continue;
                    }
                }
            }
        }

        dp[s_chars.len()][p_chars.len()]
    }
}
//leetcode submit region end(Prohibit modification and deletion)


#[test]
fn test_case() {
    assert_eq!(Solution::is_match("aa".to_string(), "a*".to_string()), true);
}