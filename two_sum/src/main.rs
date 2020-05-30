fn main() {
    let nums: Vec<i32> = vec! [3,3];
    let target: i32 = 6;
    let result: Vec<i32> = two_sum(nums, target);
    println!("{:?}", result);

    let bool = solution("abc", "bc");
    println!("{}", bool);
}

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();

    for i in 0..nums.len() {
        let b = target - nums[i];
        let mut nums2: Vec<i32> = Vec::new();
        nums2 = nums.clone();
        nums2.remove(i);

        if nums2.contains(&b){
            result.push(i as i32);
     
        }
    }
    return result;
}

fn solution(word: &str, ending: &str) -> bool {
    if word.ends_with(ending) {
        return true;
    }
    false
}