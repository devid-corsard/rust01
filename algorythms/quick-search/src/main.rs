fn main() {
    let mut arr = [4, 2, 7, 1, 9, 5, 8, 3, 6];
    quicksort(&mut arr);
    println!("{:?}", arr);
}

fn quicksort(arr: &mut [i32]) {
    if arr.len() <= 1 {
        return;
    }

    let pivot = partition(arr);

    println!("qs1 = {:?}", arr);
    quicksort(&mut arr[..pivot]);
    println!("qs2 = {:?}", arr);
    quicksort(&mut arr[pivot + 1..]);
}

fn partition(arr: &mut [i32]) -> usize {
    let pivot_index = arr.len() - 1;
    let mut i = 0;

    for j in 0..pivot_index {
        if arr[j] < arr[pivot_index] {
            println!("i = {}, j = {}", i, j);
            println!("arr = {:?}", arr);
            arr.swap(i, j);
            i += 1;
        }
    }

    arr.swap(i, pivot_index);

    i
}
