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

#[allow(dead_code)]
fn recursive_bs(arr: &[isize], target: isize) -> isize {
    if arr.is_empty() {
        return -1;
    }

    let center = arr.len() as isize / 2;
    let mid = arr[center as usize];

    if mid == target {
        center
    } else if mid > target {
        recursive_bs(
            &arr[0..center as usize], 
            target)
    } else {
        let result = recursive_bs(
            &arr[(center as usize + 1)..], 
            target);
        if result == -1 {
            -1
        } else {
            result + center + 1
        }
    }
}

#[allow(dead_code)]
fn functional_bs (arr: &[isize], target: isize) -> isize {
    match arr.binary_search(&target){
        Ok(x) => x as isize,
        Err(_) => -1, 
    }
}


#[cfg(test)]
mod tests {
    use super::*;

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

        #[test]
    fn test_recursive_bs_found() {
        let test_arr = [1, 2, 3, 4, 5, 6, 7, 8];
        assert_eq!(recursive_bs(&test_arr, 1), 0);
    }

    #[test]
    fn test_recursive_bs_not_found() {
        let test_arr = [1, 2, 3, 4, 5, 6, 7];
        assert_eq!(recursive_bs(&test_arr, 9), -1);
    }

    #[test]
    fn test_functional_bs_found() {
        let test_arr = [1, 2, 3, 4, 5, 6, 7, 8];
        assert_eq!(functional_bs(&test_arr, 1), 0);
    }

    #[test]
    fn test_functional_bs_not_found() {
        let test_arr = [1, 2, 3, 4, 5, 6, 7];
        assert_eq!(functional_bs(&test_arr, 9), -1);
    }
}