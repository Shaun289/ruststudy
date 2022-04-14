/// https://youtu.be/Ix-7qWQr_RE
///
///

pub fn cote_array_binary(nums: &mut [i32], target: i32) -> usize
{
    let mut left = 0;
    let mut right = nums.len() - 1;

    while left <= right {
        let pivot = (left + right)/2;

        if nums[pivot] == target {
            return pivot;
        }
        else if nums[pivot] < target {
            left = pivot + 1;
        }
        else { // nums[pivot] > target
            right = pivot - 1;
        }
    }
    
    return -1;
}
