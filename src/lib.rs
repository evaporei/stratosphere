use std::collections::HashMap;
use std::ops::Add;

pub fn mean<T: Add<Output = T> + Default + Copy>(data: Vec<T>) -> f64
where
    f64: std::convert::From<T>,
{
    f64::from(data.iter().fold(T::default(), |acc, curr| acc + *curr)) / data.len() as f64
}

#[derive(Debug, PartialEq)]
pub enum Mode<T> {
    None,
    Unimodal(T),
    Bimodal(T, T),
    Trimodal(T, T, T),
    Multimodal(Vec<T>),
}

use std::str::FromStr;
use std::string::ToString;

pub fn mode<T: Add + Copy + PartialEq + ToString + FromStr>(data: Vec<T>) -> Mode<T> {
    let mut values_to_frequency: HashMap<String, usize> = HashMap::new();

    for value in data {
        let new_frequency = match values_to_frequency.get_mut(&value.to_string()) {
            Some(&mut v) => v + 1,
            None => 1,
        };

        values_to_frequency.insert(value.to_string(), new_frequency);
    }

    let mut mode_result: Vec<usize> = values_to_frequency.values().cloned().collect();

    mode_result.sort();

    let mut it = mode_result.iter().rev().take_while(|v| **v != 1);

    match it.clone().count() {
        0 => Mode::None,
        1 => {
            let nxt = *it.next().unwrap();
            let key = values_to_frequency
                .iter()
                .find(|(_k, v)| **v == nxt)
                .map(|(k, _v)| k.clone())
                .unwrap();
            Mode::Unimodal(key.parse::<T>().map_err(|_e| ()).unwrap())
        }
        2 => {
            let nxt1 = *it.next().unwrap();
            let nxt2 = *it.next().unwrap();
            let key1 = values_to_frequency
                .iter()
                .find(|(_k, v)| **v == nxt1)
                .map(|(k, _v)| k.clone())
                .unwrap();
            let key2 = values_to_frequency
                .iter()
                .find(|(k, v)| **v == nxt2 && **k != key1)
                .map(|(k, _v)| k.clone())
                .unwrap();
            let mut ordered = vec![key1, key2];
            ordered.sort();
            Mode::Bimodal(
                ordered[0].parse::<T>().map_err(|_e| ()).unwrap(),
                ordered[1].parse::<T>().map_err(|_e| ()).unwrap(),
            )
        }
        3 => {
            let nxt1 = *it.next().unwrap();
            let nxt2 = *it.next().unwrap();
            let nxt3 = *it.next().unwrap();
            let key1 = values_to_frequency
                .iter()
                .find(|(_k, v)| **v == nxt1)
                .map(|(k, _v)| k.clone())
                .unwrap();
            let key2 = values_to_frequency
                .iter()
                .find(|(k, v)| **v == nxt2 && **k != key1)
                .map(|(k, _v)| k.clone())
                .unwrap();
            let key3 = values_to_frequency
                .iter()
                .find(|(k, v)| **v == nxt3 && **k != key1 && **k != key2)
                .map(|(k, _v)| k.clone())
                .unwrap();
            let mut ordered = vec![key1, key2, key3];
            ordered.sort();
            Mode::Trimodal(
                ordered[0].parse::<T>().map_err(|_e| ()).unwrap(),
                ordered[1].parse::<T>().map_err(|_e| ()).unwrap(),
                ordered[2].parse::<T>().map_err(|_e| ()).unwrap(),
            )
        }
        _ => {
            let mut result = vec![];
            let collected_modes: Vec<usize> = it.map(|a| *a).collect();
            let mut used_keys = vec![];

            for (idx, (key, value)) in values_to_frequency.iter().enumerate() {
                if collected_modes[idx] == *value {
                    if used_keys.contains(key) {
                        continue;
                    }

                    used_keys.push(key.clone());
                    result.push(key.clone());
                }
            }

            result.sort();

            Mode::Multimodal(
                result
                    .iter()
                    .map(|v| v.parse::<T>().map_err(|_e| ()).unwrap())
                    .collect(),
            )
        }
    }
}

pub fn median<T: Copy + Default + Add<Output = T>>(data: Vec<T>) -> Option<f64>
where
    f64: std::convert::From<T>,
{
    let is_odd = data.len() % 2 != 0;
    if data.len() == 0 {
        None
    } else if is_odd {
        Some(f64::from(data[data.len() / 2]))
    } else {
        let middle1 = data[data.len() / 2 - 1];
        let middle2 = data[data.len() / 2];
        let middle_mean = mean(vec![middle1.into(), middle2.into()]);
        Some(middle_mean)
    }
}

pub fn variance(data: Vec<i32>) -> f64 {
    let list_mean = mean(data.clone());
    let it = data.iter();
    let differences = it.map(|v| (f64::from(*v) - list_mean).abs());
    let powered_differences = differences.map(|d| d * d);
    let powered_differences: Vec<f64> = powered_differences.collect();
    mean(powered_differences)
}

#[cfg(test)]
mod tests {
    use crate::{mean, median, mode, Mode};

    #[test]
    fn test_mean_positive_integers() {
        let result = mean(vec![0, 1, 2, 3, 4, 5]);
        assert_eq!(result, 2.5);
    }

    #[test]
    fn test_mean_negative_integers() {
        let result = mean(vec![0, -1, -2, -3, -4, -5]);
        assert_eq!(result, -2.5);
    }

    #[test]
    fn test_mean_mixed_integers() {
        let result = mean(vec![0, -1, 2, -3, 4, -5]);
        assert_eq!(result, -0.5);
    }

    #[test]
    fn test_mean_positive_floats() {
        let result = mean(vec![0.4, 1.2, 2.8, 3.3, 4.9, 5.2]);
        assert_eq!(result, 2.966666666666667);
    }

    #[test]
    fn test_mode_positive_integers_none() {
        let result = mode(vec![0, 1, 2, 3, 4, 5]);
        assert_eq!(result, Mode::None);
    }

    #[test]
    fn test_mode_none_with_empty_list() {
        let empty_list: Vec<usize> = vec![];
        let result = mode(empty_list);
        assert_eq!(result, Mode::None);
    }

    #[test]
    fn test_mode_positive_integers_unimodal() {
        let result = mode(vec![0, 1, 2, 3, 1]);
        assert_eq!(result, Mode::Unimodal(1));
    }

    #[test]
    fn test_mode_positive_integers_bimodal() {
        let result = mode(vec![0, 1, 3, 3, 1]);
        assert_eq!(result, Mode::Bimodal(1, 3));
    }

    #[test]
    fn test_mode_positive_integers_trimodal() {
        let result = mode(vec![0, 1, 3, 3, 1, 0]);
        assert_eq!(result, Mode::Trimodal(0, 1, 3));
    }

    #[test]
    fn test_mode_positive_integers_multimodal() {
        let result = mode(vec![0, 1, 3, 3, 1, 0, 2, 2]);
        assert_eq!(result, Mode::Multimodal(vec![0, 1, 2, 3]));
    }

    #[test]
    fn test_mode_negative_integers_none() {
        let result = mode(vec![0, -1, -2, -3, -4, -5]);
        assert_eq!(result, Mode::None);
    }

    #[test]
    fn test_mode_negative_integers_unimodal() {
        let result = mode(vec![0, -1, -2, -3, -1]);
        assert_eq!(result, Mode::Unimodal(-1));
    }

    #[test]
    fn test_mode_negative_integers_bimodal() {
        let result = mode(vec![0, -1, -3, -3, -1]);
        assert_eq!(result, Mode::Bimodal(-1, -3));
    }

    #[test]
    fn test_mode_negative_integers_trimodal() {
        let result = mode(vec![0, -1, -3, -3, -1, 0]);
        assert_eq!(result, Mode::Trimodal(-1, -3, 0));
    }

    #[test]
    fn test_mode_negative_integers_multimodal() {
        let result = mode(vec![-0, -1, -3, -3, -1, 0, -2, -2]);
        assert_eq!(result, Mode::Multimodal(vec![-1, -2, -3, 0]));
    }

    #[test]
    fn test_mode_mixed_integers_unimodal() {
        let result = mode(vec![0, -1, 2, 3, -1]);
        assert_eq!(result, Mode::Unimodal(-1));
    }

    #[test]
    fn test_mode_floats_unimodal() {
        let result = mode(vec![0.0, 1.0, 2.0, 3.0, 1.0]);
        assert_eq!(result, Mode::Unimodal(1.0));
    }

    #[test]
    fn test_mode_floats_bimodal() {
        let result = mode(vec![0.0, 1.5, 3.2, 3.2, 1.5]);
        assert_eq!(result, Mode::Bimodal(1.5, 3.2));
    }

    #[test]
    fn test_mode_floats_trimodal() {
        let result = mode(vec![0.4, 1.1, 3.9, 3.9, 1.1, 0.4]);
        assert_eq!(result, Mode::Trimodal(0.4, 1.1, 3.9));
    }

    #[test]
    fn test_mode_floats_multimodal() {
        let result = mode(vec![0.3, 1.7, 3.5, 3.5, 1.7, 0.3, 2.6, 2.6]);
        assert_eq!(result, Mode::Multimodal(vec![0.3, 1.7, 2.6, 3.5]));
    }

    #[test]
    fn test_median_empty_list() {
        let empty_list: Vec<i32> = vec![];
        let result = median(empty_list);
        assert_eq!(result, None);
    }

    #[test]
    fn test_median_odd_length_positive_integers() {
        let result = median(vec![7, 8, 3, 9, 22]);
        assert_eq!(result, Some(3.0));
    }

    #[test]
    fn test_median_even_length_positive_integers() {
        let result = median(vec![7, 8, 3, 6, 22, 42]);
        assert_eq!(result, Some(4.5));
    }

    #[test]
    fn test_median_odd_length_negative_integers() {
        let result = median(vec![-7, -8, -3, -9, -22]);
        assert_eq!(result, Some(-3.0));
    }

    #[test]
    fn test_median_even_length_negative_integers() {
        let result = median(vec![-7, -8, -3, -6, -22, -42]);
        assert_eq!(result, Some(-4.5));
    }

    #[test]
    fn test_median_even_length_mixed_integers() {
        let result = median(vec![7, -8, -3, 6, -22, -42]);
        assert_eq!(result, Some(1.5));
    }
}
