#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_push_smaller_then_larger_true_if_swapped() {
        let input_list: Vec<i32> = vec![];
        let a = 5;
        let b = 9;
        let if_not_swapped = false;

        let expected_list = vec![5, 9];
        let expected_swapped_status = false;

        let (res_list, swapped_status) =
            push_smaller_then_larger_true_if_swapped(input_list, a, b, if_not_swapped);
        assert_eq!(res_list, expected_list);
        assert_eq!(swapped_status, expected_swapped_status);
    }

    #[test]
    fn test_push_smaller_then_larger_true_if_swapped2() {
        let input_list: Vec<i32> = vec![];
        let a = 9;
        let b = 5;
        let if_not_swapped = false;

        let expected_list = vec![5, 9];
        let expected_swapped_status = true;

        let (res_list, swapped_status) =
            push_smaller_then_larger_true_if_swapped(input_list, a, b, if_not_swapped);
        assert_eq!(res_list, expected_list);
        assert_eq!(swapped_status, expected_swapped_status);
    }

    #[test]
    fn test_push_smaller_then_larger_true_if_swapped3() {
        let input_list: Vec<i32> = vec![];
        let a = 5;
        let b = 9;
        let if_not_swapped = true;

        let expected_list = vec![5, 9];
        let expected_swapped_status = true;

        let (res_list, swapped_status) =
            push_smaller_then_larger_true_if_swapped(input_list, a, b, if_not_swapped);
        assert_eq!(res_list, expected_list);
        assert_eq!(swapped_status, expected_swapped_status);
    }
}

// fn bubble_sort(list: &mut Vec<i32>) {
//     let n = list.len();
//     loop {
//         let mut swapped = false;
//         for i in 1..=n - 1 {
//             if list[i - 1] > list[i] {
//                 (list[i - 1], list[i]) = (list[i], list[i - 1]);
//                 swapped = true;
//             }
//         }
//         if !swapped {
//             break;
//         }
//     }
// }

fn bubble_sort<T: std::cmp::PartialOrd + Copy>(list: &mut Vec<T>) {
    let n = list.len();
    loop {
        let mut swapped = false;
        for i in 1..=n - 1 {
            if list[i - 1] > list[i] {
                (list[i - 1], list[i]) = (list[i], list[i - 1]);
                swapped = true;
            }
        }
        if !swapped {
            break;
        }
    }
}

fn functional_push<T: Clone>(list: Vec<T>, value: T) -> Vec<T> {
    let mut new_list = list.clone();
    new_list.push(value);
    new_list
}

fn push_smaller_then_larger_true_if_swapped<T: std::cmp::PartialOrd + Clone>(
    list: Vec<T>,
    a: T,
    b: T,
    if_not_swapped: bool,
) -> (Vec<T>, bool) {
    let (smaller, larger, swapped) = if a <= b {
        (a, b, if_not_swapped)
    } else {
        (b, a, true)
    };
    (
        functional_push(functional_push(list, smaller), larger),
        swapped,
    )
}

fn get_list_without_last_and_last<T: Copy>(list: Vec<T>) -> (Vec<T>, T) {
    let last = list[list.len() - 1];
    let list_len = list.len();
    let list_without_last: Vec<T> = list.into_iter().take(list_len - 1).collect();
    (list_without_last, last)

    // (vec![], 0)
}

fn bubble_sort_inner_loop<T: std::cmp::PartialOrd + Copy>(list: Vec<T>) -> (Vec<T>, bool) {
    let res = list
        .into_iter()
        .map(|x| (vec![x], false)) // second tuple element is "has_swapped" info
        .reduce(|(acc, has_swapped), (x, _x_has_swapped)| {
            let x_value = x[0];
            if acc.is_empty() {
                return (x, _x_has_swapped);
            }
            let (list_without_last, current_last) = get_list_without_last_and_last(acc);
            push_smaller_then_larger_true_if_swapped(
                list_without_last,
                current_last,
                x_value,
                has_swapped,
            )
        })
        .unwrap();
    res
}

fn bubble_sort3<T: std::cmp::PartialOrd + Copy>(list: Vec<T>) -> Vec<T> {
    fn bubble_sort3_rec<T: std::cmp::PartialOrd + Copy>(list: Vec<T>) -> Vec<T> {
        let (new_list, has_swapped) = bubble_sort_inner_loop(list);
        if !has_swapped {
            return new_list;
        }
        bubble_sort3_rec(new_list)
    }
    bubble_sort3_rec(list)
}

fn bubble_sort2<T: std::cmp::PartialOrd + Clone + Copy>(list: Vec<T>) -> Vec<T> {
    fn bubble_sort2_rec<T: std::cmp::PartialOrd + Clone + Copy>(
        list: Vec<T>,
        has_swapped: bool,
    ) -> Vec<T> {
        if !has_swapped {
            return list;
        }
        let mut res_list = list.clone();
        let n = list.len();
        let mut swapped = false;
        for i in 1..=n - 1 {
            if list[i - 1] > list[i] {
                (res_list[i - 1], res_list[i]) = (res_list[i], res_list[i - 1]);
                swapped = true;
            }
        }
        bubble_sort2_rec(res_list, swapped)
    }
    bubble_sort2_rec(list, true)
}

fn main() {
    let my_list = vec![5, 9, 2, 1, 5, 3, 5, 8, 2, 3];

    let mut sorted = my_list.clone();
    sorted.sort();
    let mut my_sorted = my_list.clone();
    bubble_sort(&mut my_sorted);
    println!("{:?}", sorted);
    println!("{:?}", my_sorted);
    assert_eq!(my_sorted, sorted);

    println!("testing bubble_sort2");

    let bubble_sorted2 = bubble_sort2(my_list.clone());
    println!("{:?}", bubble_sorted2);
    assert_eq!(bubble_sorted2, sorted);

    let bubble_sorted3 = bubble_sort3(my_list.clone());
    println!("{:?}", bubble_sorted3);
    assert_eq!(bubble_sorted3, sorted);

    println!("bye");
}
