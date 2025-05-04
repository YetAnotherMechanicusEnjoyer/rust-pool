pub fn task06(arr: &mut [i32], size: usize) {
    for i in 1..size {
        let index = size - i;
        let mut sorted: bool = true;
        for j in 0..index {
            if arr[j + 1] < arr[j] {
                arr.swap(j, j + 1);
                sorted = false;
            }
        }
        if sorted {
            break;
        }
    }
}
