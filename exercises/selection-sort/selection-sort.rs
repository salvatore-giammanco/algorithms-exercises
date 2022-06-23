fn selection_sort(to_sort: &mut[u32]) {
    for i in 0..to_sort.len() {
        let mut min_index = i;
        for j in i..to_sort.len() {
            if to_sort[j] < to_sort[min_index] {
                min_index = j;
            }
        }
        to_sort.swap(i, min_index);
    }
}


fn main() {
    let mut input = vec![92, 3, 199, 9, 10, 23, 42, 0, 71, 1];
    selection_sort(&mut input);
    println!("{:?}", input);
}