use std::collections::HashMap;

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map = HashMap::new();
    for (index, num) in nums.iter().enumerate() {
        let diff = target - num;
        match map.get(&diff) {
            Some(&i) => return vec![i as i32, index as i32],
            None => {
                map.insert(num, index);
            }
        }
    }
    vec![]
}

pub fn two_sum_two(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    let mut lp: usize = 0;
    let mut rp: usize = numbers.len() - 1;

    while lp < rp {
        let sum = numbers[lp] + numbers[rp];

        if sum == target {
            return vec![lp as i32 + 1, rp as i32 + 1];
        } else if sum < target {
            lp += 1;
        } else {
            rp -= 1;
        }
    }
    vec![]
}

fn main() {}

