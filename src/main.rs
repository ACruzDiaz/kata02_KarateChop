fn main() {
    println!("Hello, world!");

}

#[allow(dead_code)]
fn imperative_bs(arr: &[isize], target: isize) -> isize {
    let mut l: isize = 0;
    let mut r: isize = arr.len() as isize - 1;

    while l <= r {
        let center = (l + r) / 2;          
        let mid = arr[center as usize];    

        if mid == target {
            return center;                 
        } else if mid > target {
            r = center - 1;
        } else {
            l = center + 1;
        }
    }
    -1
}



#[cfg(test)]
mod tests {
    use super::imperative_bs;

    #[test]
    fn test_imperative_bs_found() {
        let test_arr = [1, 2, 3, 4, 5, 6, 7];
        assert_eq!(imperative_bs(&test_arr, 5), 4);
    }

    #[test]
    fn test_imperative_bs_not_found() {
        let test_arr = [1, 2, 3, 4, 5, 6, 7];
        assert_eq!(imperative_bs(&test_arr, 9), -1);
    }
}