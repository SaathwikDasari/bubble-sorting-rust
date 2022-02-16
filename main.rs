fn main() {
    let mut arr: [i32; 5] = [34, 412, 41, 543, 1];

    for i in 0..arr.len() {
        for j in ((i+1)..arr.len()).rev() {
            if arr[j] < arr[j-1] {
                swap_item(&mut arr, j);
            }
        }
    }

    println!("{:?}", arr);
}

fn swap_item(array: &mut [i32;5], i:usize) {
    let temp;

    temp = array[i];
    array[i] = array[i-1];
    array[i-1] = temp;
}
