use crate::mean::mean;
use crate::number::Number;

pub fn variance<T: Number>(data: &Vec<T>) -> f64 {
    let list_mean = mean(&data);
    let it = data.iter();
    let differences = it.map(|v| ((*v).into() - list_mean).abs());
    let powered_differences = differences.map(|d| d * d);
    let powered_differences: Vec<f64> = powered_differences.collect();
    mean(&powered_differences)
}

#[test]
fn test_variance_positive_integers() {
    let result = variance(&vec![14, 22, 19, 21, 21]);
    assert_eq!(result, 8.239999999999998);
}

#[test]
fn test_variance_negative_integers() {
    let result = variance(&vec![-22, -21, -20, -18, -15]);
    assert_eq!(result, 6.159999999999999);
}

#[test]
fn test_variance_positive_floats() {
    let result = variance(&vec![7.5, 6.0, 7.0, 6.5, 8.0]);
    assert_eq!(result, 0.5);
}

#[test]
fn test_variance_negative_floats() {
    let result = variance(&vec![-7.5, -5.0, -8.0, -8.3, -6.0]);
    assert_eq!(result, 1.5864000000000005);
}

#[test]
fn test_variance_mixed_floats() {
    let result = variance(&vec![-7.5, 5.0, -8.0, 8.3, -6.0]);
    assert_eq!(result, 47.33840000000001);
}
