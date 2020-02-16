use crate::number::Number;
use crate::variance::variance;

pub fn standard_deviation<T: Number>(data: &Vec<T>) -> f64 {
    variance(&data.iter().map(|a| (*a).into()).collect()).sqrt()
}

#[test]
fn test_standard_deviation_positive_integers() {
    let result = standard_deviation(&vec![14, 22, 19, 21, 21]);
    assert_eq!(result, 2.8705400188814645);
}

#[test]
fn test_standard_deviation_negative_integers() {
    let result = standard_deviation(&vec![-22, -21, -20, -18, -15]);
    assert_eq!(result, 2.481934729198171);
}

#[test]
fn test_standard_deviation_positive_floats() {
    let result = standard_deviation(&vec![7.5, 6.0, 7.0, 6.5, 8.0]);
    assert_eq!(result, 0.7071067811865476);
}

#[test]
fn test_standard_deviation_negative_floats() {
    let result = standard_deviation(&vec![-7.5, -5.0, -8.0, -8.3, -6.0]);
    assert_eq!(result, 1.2595237195067033);
}

#[test]
fn test_standard_deviation_mixed_floats() {
    let result = standard_deviation(&vec![-7.5, 5.0, -8.0, 8.3, -6.0]);
    assert_eq!(result, 6.880290691533316);
}
