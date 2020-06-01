fn main() {
    let mut nums: Vec<i32> = vec![];
    let len = remove_duplicates(&mut nums);

    println!("{}", len);
    println!("{:?}",nums);
}

fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    if nums.len() == 0 {
        return 0 
    }
    let mut i = 1;
    loop {
        if nums.get(i as usize).is_none() {
            break i
        }
        if nums.get(i as usize) == nums.get(i as usize - 1) {
            nums.remove(i as usize);
            continue
        }
        i = i + 1;
    }
}