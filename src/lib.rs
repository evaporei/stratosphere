mod mean;
mod median;
mod mode;
mod number;

pub use mean::mean;
pub use median::median;
pub use mode::mode;
use number::Number;

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
    use crate::{standard_deviation, variance};

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
