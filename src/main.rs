include!("../editor/cn/add_two_numbers.rs");

fn main(){
    let l1 = get_list(342);
    let l2 = get_list(465);
    let ans = Solution::add_two_numbers(Some(l1), Some(l2));
    println!("{}", get_num(ans));
}