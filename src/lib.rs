mod mean;
mod mode;
mod number;

pub use mean::mean;
pub use mode::mode;
use number::Number;

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

pub fn variance<T: Number>(data: &Vec<T>) -> f64 {
    let list_mean = mean(&data);
    let it = data.iter();
    let differences = it.map(|v| ((*v).into() - list_mean).abs());
    let powered_differences = differences.map(|d| d * d);
    let powered_differences: Vec<f64> = powered_differences.collect();
    mean(&powered_differences)
}

pub fn standard_deviation<T: Number>(data: &Vec<T>) -> f64 {
    variance(&data.iter().map(|a| (*a).into()).collect()).sqrt()
}

#[cfg(test)]
mod tests {
    use crate::{median, standard_deviation, variance};

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
}
