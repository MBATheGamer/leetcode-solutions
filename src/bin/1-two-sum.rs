struct Solution;

impl Solution {
  pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut vector: Vec<i32> = vec![];
    for i in 0..nums.len() - 1 {
      if nums[i] + nums[i + 1] == target {
        vector.push(i as i32);
        vector.push(i as i32 + 1);
        return vector;
      }
    }
    return vector;
  }
}

fn main() {
  let nums = vec![2, 7, 11, 15];
  println!("{:?}", Solution::two_sum(nums, 9));
  let nums = vec![3, 2, 4];
  println!("{:?}", Solution::two_sum(nums, 6));
  let nums = vec![3, 3];
  println!("{:?}", Solution::two_sum(nums, 6));
}
