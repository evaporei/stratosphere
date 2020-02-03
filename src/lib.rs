use std::collections::HashMap;
use std::ops::Add;

pub fn mean<T: Add<Output = T> + Default + Copy>(data: Vec<T>) -> f64
where
    f64: std::convert::From<T>,
{
    f64::from(data.iter().fold(T::default(), |acc, curr| acc + *curr)) / data.len() as f64
}

#[derive(Debug, PartialEq)]
pub enum Mode {
    None,
    Unimodal(isize),
    Bimodal(isize, isize),
    Trimodal(isize, isize, isize),
    Multimodal(Vec<isize>),
}

pub fn mode(data: Vec<isize>) -> Mode {
    let mut values_to_frequency: HashMap<isize, usize> = HashMap::new();

    for value in data {
        let new_frequency = match values_to_frequency.get_mut(&value) {
            Some(&mut v) => v + 1,
            None => 1,
        };

        values_to_frequency.insert(value, new_frequency);
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
                .map(|(k, _v)| *k)
                .unwrap();
            Mode::Unimodal(key)
        }
        2 => {
            let nxt1 = *it.next().unwrap();
            let nxt2 = *it.next().unwrap();
            let key1 = values_to_frequency
                .iter()
                .find(|(_k, v)| **v == nxt1)
                .map(|(k, _v)| *k)
                .unwrap();
            let key2 = values_to_frequency
                .iter()
                .find(|(k, v)| **v == nxt2 && **k != key1)
                .map(|(k, _v)| *k)
                .unwrap();
            let mut ordered = vec![key1, key2];
            ordered.sort();
            Mode::Bimodal(ordered[0], ordered[1])
        }
        3 => {
            let nxt1 = *it.next().unwrap();
            let nxt2 = *it.next().unwrap();
            let nxt3 = *it.next().unwrap();
            let key1 = values_to_frequency
                .iter()
                .find(|(_k, v)| **v == nxt1)
                .map(|(k, _v)| *k)
                .unwrap();
            let key2 = values_to_frequency
                .iter()
                .find(|(k, v)| **v == nxt2 && **k != key1)
                .map(|(k, _v)| *k)
                .unwrap();
            let key3 = values_to_frequency
                .iter()
                .find(|(k, v)| **v == nxt3 && **k != key1 && **k != key2)
                .map(|(k, _v)| *k)
                .unwrap();
            let mut ordered = vec![key1, key2, key3];
            ordered.sort();
            Mode::Trimodal(ordered[0], ordered[1], ordered[2])
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

                    used_keys.push(*key);
                    result.push(*key);
                }
            }

            result.sort();

            Mode::Multimodal(result)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{mean, mode, Mode};

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
        let result = mode(vec![]);
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
        assert_eq!(result, Mode::Bimodal(-3, -1));
    }

    #[test]
    fn test_mode_negative_integers_trimodal() {
        let result = mode(vec![0, -1, -3, -3, -1, 0]);
        assert_eq!(result, Mode::Trimodal(-3, -1, 0));
    }

    #[test]
    fn test_mode_negative_integers_multimodal() {
        let result = mode(vec![-0, -1, -3, -3, -1, 0, -2, -2]);
        assert_eq!(result, Mode::Multimodal(vec![-3, -2, -1, 0]));
    }

    #[test]
    fn test_mode_mixed_integers_unimodal() {
        let result = mode(vec![0, -1, 2, 3, -1]);
        assert_eq!(result, Mode::Unimodal(-1));
    }
}
