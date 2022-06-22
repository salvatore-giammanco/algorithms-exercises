fn search(nums: &[u32], target: u32) -> i32 {
    let mut left_pointer: u16 = 0;
    let mut right_pointer: u16 = (nums.len() - 1) as u16;

    if right_pointer == 0 {
        return if nums[0] == target {0} else {-1};
    }
    
    while left_pointer <= right_pointer {
        let middle_pointer: usize = ((left_pointer + right_pointer) / 2).into();
        let guess = nums[middle_pointer];

        if guess == target {
            return middle_pointer as i32;
        }
        if guess > target {
            right_pointer = middle_pointer as u16 - 1;
        } else {
            left_pointer = middle_pointer as u16 + 1;
        }

    }
    -1
}

fn main() {
    let result: i32 = search(&vec![2, 3, 5, 9, 10, 23, 42, 71], 42);
    println!("result: {}", result);
}