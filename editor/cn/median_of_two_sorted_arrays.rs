//给定两个大小为 m 和 n 的正序（从小到大）数组 nums1 和 nums2。 
//
// 请你找出这两个正序数组的中位数，并且要求算法的时间复杂度为 O(log(m + n))。 
//
// 你可以假设 nums1 和 nums2 不会同时为空。 
//
// 
//
// 示例 1: 
//
// nums1 = [1, 3]
//nums2 = [2]
//
//则中位数是 2.0
// 
//
// 示例 2: 
//
// nums1 = [1, 2]
//nums2 = [3, 4]
//
//则中位数是 (2 + 3)/2 = 2.5
// 
// Related Topics 数组 二分查找 分治算法 
// 👍 3225 👎 0


pub struct Solution {}

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        if nums1.len() == 1 && nums2.len() == 0 {
            return nums1[0] as f64;
        }
        if nums1.len() == 0 && nums2.len() == 1 {
            return nums2[0] as f64;
        }

        let mut pointer_a = 0;
        let mut pointer_b = 0;
        //flag 用于记录最后一次移动的 是 pointer_a 或者 是 pointer_b 或者都移动了
        let mut flag = 0;
        let mut lastnum = 0;
        loop {
            // 这个判断取的是公共长度的 部分 a 如果 长度为 5 b如果长度为 4 那么 这个判断里面 是 前 4个数归并
            if pointer_a < nums1.len() && pointer_b < nums2.len() {
                if nums1[pointer_a] < nums2[pointer_b] { //如果 指针a的值 <  指针b的值 指针 a 往后移动一个位置
                    flag = 0;
                } else if nums1[pointer_a] > nums2[pointer_b] {  //如果 指针a的值 >  指针b的值 指针 b 往后移动一个位
                    flag = 1;
                } else if nums1[pointer_a] == nums2[pointer_b] {//如果两个数 相等 2根指针 都往后移动1个位置
                    flag = 2;
                }
            } else if pointer_a < nums1.len() { //当 b指针的 长度 耗尽 了 我们只需要移动 a 指针就好了
                flag = 0;
            } else if pointer_b < nums2.len() { // 当 a指针的 长度耗尽了 ,接下来 我们只需要移动b 指针了
                flag = 1;
            } else {
                break;
            }

            //如果 指针 指向了 中位数
            if ((nums1.len() + nums2.len() - 2) / 2) + 1 == pointer_a + pointer_b {
                //如果 数组1 + 数组2 是奇数个
                if (nums1.len() + nums2.len()) % 2 != 0 {
                    //处理 指针 a 移动了的情况
                    if flag == 0 {
                        return nums1[pointer_a] as f64;
                    } else {
                        //如果指针 b 或者 2个指针都移动 的 情况
                        return nums2[pointer_b] as f64;
                    }
                } else {//如果 数组1 + 数组2 偶数个

                    //如果 是 偶数个 我们 需要 记录上一个数 和当前的 数相加 / 2
                    if flag == 0 {
                        return (nums1[pointer_a] as f64 + lastnum as f64) / 2.00;
                    } else {
                        return (nums2[pointer_b] as f64 + lastnum as f64) / 2.00;
                    }
                }
                //处理 pointer_a 和 【pointer_b 相等 都往后 移动 2步,那么 pointer_a + pointer_b == 数组1长度 + 数组2长度 -2 (从0索引开始 所以 -1 -1 = -2) /2 + 1
            } else if (nums1.len() + nums2.len() - 2) / 2 + 1 < pointer_a + pointer_b {
                break;
            } else { //如果 不是中位数
                if flag == 0 {
                    lastnum = nums1[pointer_a];
                    //指针 a 往后移动一步
                    pointer_a += 1;
                } else if flag == 1 {
                    lastnum = nums2[pointer_b];
                    //指针 b 往后移动一步
                    pointer_b += 1;
                } else {
                    lastnum = nums1[pointer_a];
                    //指针 a b 分别往后移动一步
                    pointer_a += 1;
                    pointer_b += 1;
                }
            }
        }
        return lastnum as f64;
    }
}
//leetcode submit region end(Prohibit modification and deletion)


#[test]
fn test_case() {}