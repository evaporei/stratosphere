use crate::number::Number;

pub fn mean<T: Number>(data: &[T]) -> f64 {
    (data.iter().fold(T::default(), |acc, curr| acc + *curr)).into() / data.len() as f64
}

#[test]
fn test_mean_positive_integers() {
    let result = mean(&[0, 1, 2, 3, 4, 5]);
    assert_eq!(result, 2.5);
}

#[test]
fn test_mean_negative_integers() {
    let result = mean(&[0, -1, -2, -3, -4, -5]);
    assert_eq!(result, -2.5);
}

#[test]
fn test_mean_mixed_integers() {
    let result = mean(&[0, -1, 2, -3, 4, -5]);
    assert_eq!(result, -0.5);
}

#[test]
fn test_mean_positive_floats() {
    let result = mean(&[0.4, 1.2, 2.8, 3.3, 4.9, 5.2]);
    assert_eq!(result, 2.966666666666667);
}
