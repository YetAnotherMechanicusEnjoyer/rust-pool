pub fn task06(arr: &mut Vec<i32>, size: usize) {
    for i in 0..size {
        for j in 0..i {
            if arr[j + 1] < arr[j] {
                let tmp = arr[j];
                arr[j] = arr[j + 1];
                arr[j + 1] = tmp;
            }
        }
    }
}
