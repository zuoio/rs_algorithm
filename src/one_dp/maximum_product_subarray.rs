pub fn max_product(nums: Vec<i32>) -> i32 {
    let mut res = nums[0];
    let mut cur_max = 1;
    let mut cur_min = 1;

    for num in nums {
        let temp = cur_max * num;
        cur_max = num.max(temp.max(cur_min * num));
        cur_min = num.min(temp.min(cur_min * num));
        res = res.max(cur_max);
    }
    return res;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_product_case1() {
        let nums = vec![2, 3, -2, 4];
        assert_eq!(max_product(nums), 6);
    }

    #[test]
    fn test_max_product_case2() {
        let nums = vec![-2, 0, -1];
        assert_eq!(max_product(nums), 0);
    }
}
