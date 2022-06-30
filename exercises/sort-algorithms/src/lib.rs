pub fn insertion_sort(to_sort: &mut Vec<u32>) {
    for i in 0..to_sort.len() {
        let mut j: usize = i;
        while j > 0 && to_sort[j] < to_sort[j - 1] {
            to_sort.swap(j, j - 1);
            j -= 1;
        }
    }
}

pub fn bubble_sort(to_sort: &mut Vec<u32>) {
    let len = to_sort.len();
    for i in (1..len).rev() {
        for j in (len - i..len).rev() {
            if to_sort[j - 1] > to_sort[j] {
                to_sort.swap(j, j - 1);
            }
        }
    }
}

pub fn selection_sort(to_sort: &mut Vec<u32>) {
    let len = to_sort.len();
    for i in 0..len {
        let mut min_index = i;
        for j in i..len {
            if to_sort[j] < to_sort[min_index] {
                min_index = j;
            }
        }
        to_sort.swap(i, min_index);
    }
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
        let mut test_data = test_setup();
        insertion_sort(&mut test_data.to_sort);
        
        assert_eq!(test_data.to_sort, test_data.sorted);
    }

    #[test]
    fn test_bubble_sort() {
        let mut test_data = test_setup();
        bubble_sort(&mut test_data.to_sort);
        
        assert_eq!(test_data.to_sort, test_data.sorted);
    }

    #[test]
    fn test_selection_sort() {
        let mut test_data = test_setup();
        selection_sort(&mut test_data.to_sort);
        
        assert_eq!(test_data.to_sort, test_data.sorted);
    }
}