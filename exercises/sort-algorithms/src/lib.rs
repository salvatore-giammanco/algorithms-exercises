pub fn insertion_sort(to_sort: &Vec<u32>) -> Vec<u32> {
    let mut sorted = to_sort.clone();

    for i in 0..sorted.len() {
        let mut j: usize = i;
        while j > 0 && sorted[j] < sorted[j - 1] {
            sorted.swap(j, j - 1);
            j -= 1;
        }
    }
    sorted
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestData {
        to_sort: Vec<u32>,
        sorted: Vec<u32>,
    }

    fn test_setup() -> TestData {
        TestData {
            to_sort: vec![14, 87, 42, 21, 12, 22, 6, 0, 103, 66],
            sorted: vec![0, 6, 12, 14, 21, 22, 42, 66, 87, 103],
        }
    }
    
    #[test]
    fn test_insertion_sort() {
        let test_data = test_setup();
        let sorted = insertion_sort(&test_data.to_sort);
        println!("{:?}", sorted);
        assert_eq!(sorted, test_data.sorted);
    }
}