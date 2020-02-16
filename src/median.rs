use crate::mean::mean;
use crate::number::Number;

pub fn median<T: Number>(data: &[T]) -> Option<f64> {
    let is_odd = data.len() % 2 != 0;
    if data.len() == 0 {
        None
    } else if is_odd {
        Some((data[data.len() / 2]).into())
    } else {
        let middle1 = data[data.len() / 2 - 1];
        let middle2 = data[data.len() / 2];
        let middle_mean = mean(&[middle1.into(), middle2.into()]);
        Some(middle_mean)
    }
}

#[test]
fn test_median_empty_list() {
    let empty_list: Vec<i32> = vec![];
    let result = median(&empty_list);
    assert_eq!(result, None);
}

#[test]
fn test_median_odd_length_positive_integers() {
    let result = median(&[7, 8, 3, 9, 22]);
    assert_eq!(result, Some(3.0));
}

#[test]
fn test_median_even_length_positive_integers() {
    let result = median(&[7, 8, 3, 6, 22, 42]);
    assert_eq!(result, Some(4.5));
}

#[test]
fn test_median_odd_length_negative_integers() {
    let result = median(&[-7, -8, -3, -9, -22]);
    assert_eq!(result, Some(-3.0));
}

#[test]
fn test_median_even_length_negative_integers() {
    let result = median(&[-7, -8, -3, -6, -22, -42]);
    assert_eq!(result, Some(-4.5));
}

#[test]
fn test_median_even_length_mixed_integers() {
    let result = median(&[7, -8, -3, 6, -22, -42]);
    assert_eq!(result, Some(1.5));
}
