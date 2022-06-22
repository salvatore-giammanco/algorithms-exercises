
fn bubble_sort(to_sort: &mut [u32]) {
    // In place bubble sort algorithm
    let len: usize = to_sort.len();
    for i in (1..len).rev() {
        for j in (len - i..len).rev() {
            if to_sort[j - 1] > to_sort[j] {
                to_sort.swap(j, j - 1);
            }
        }
    }
}

fn main() {
    let mut input = vec![92, 3, 199, 9, 10, 23, 42, 0, 71, 1];
    bubble_sort(&mut input);
    println!("{:?}", input);
}