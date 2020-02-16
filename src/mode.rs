use crate::number::Number;
use std::collections::HashMap;

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

pub fn mode<T: Number + ToString + FromStr>(data: &[T]) -> Mode<T> {
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

#[test]
fn test_mode_positive_integers_none() {
    let result = mode(&[0, 1, 2, 3, 4, 5]);
    assert_eq!(result, Mode::None);
}

#[test]
fn test_mode_none_with_empty_list() {
    let empty_list: Vec<i32> = vec![];
    let result = mode(&empty_list);
    assert_eq!(result, Mode::None);
}

#[test]
fn test_mode_positive_integers_unimodal() {
    let result = mode(&[0, 1, 2, 3, 1]);
    assert_eq!(result, Mode::Unimodal(1));
}

#[test]
fn test_mode_positive_integers_bimodal() {
    let result = mode(&[0, 1, 3, 3, 1]);
    assert_eq!(result, Mode::Bimodal(1, 3));
}

#[test]
fn test_mode_positive_integers_trimodal() {
    let result = mode(&[0, 1, 3, 3, 1, 0]);
    assert_eq!(result, Mode::Trimodal(0, 1, 3));
}

#[test]
fn test_mode_positive_integers_multimodal() {
    let result = mode(&[0, 1, 3, 3, 1, 0, 2, 2]);
    assert_eq!(result, Mode::Multimodal(vec![0, 1, 2, 3]));
}

#[test]
fn test_mode_negative_integers_none() {
    let result = mode(&[0, -1, -2, -3, -4, -5]);
    assert_eq!(result, Mode::None);
}

#[test]
fn test_mode_negative_integers_unimodal() {
    let result = mode(&[0, -1, -2, -3, -1]);
    assert_eq!(result, Mode::Unimodal(-1));
}

#[test]
fn test_mode_negative_integers_bimodal() {
    let result = mode(&[0, -1, -3, -3, -1]);
    assert_eq!(result, Mode::Bimodal(-1, -3));
}

#[test]
fn test_mode_negative_integers_trimodal() {
    let result = mode(&[0, -1, -3, -3, -1, 0]);
    assert_eq!(result, Mode::Trimodal(-1, -3, 0));
}

#[test]
fn test_mode_negative_integers_multimodal() {
    let result = mode(&[-0, -1, -3, -3, -1, 0, -2, -2]);
    assert_eq!(result, Mode::Multimodal(vec![-1, -2, -3, 0]));
}

#[test]
fn test_mode_mixed_integers_unimodal() {
    let result = mode(&[0, -1, 2, 3, -1]);
    assert_eq!(result, Mode::Unimodal(-1));
}

#[test]
fn test_mode_floats_unimodal() {
    let result = mode(&[0.0, 1.0, 2.0, 3.0, 1.0]);
    assert_eq!(result, Mode::Unimodal(1.0));
}

#[test]
fn test_mode_floats_bimodal() {
    let result = mode(&[0.0, 1.5, 3.2, 3.2, 1.5]);
    assert_eq!(result, Mode::Bimodal(1.5, 3.2));
}

#[test]
fn test_mode_floats_trimodal() {
    let result = mode(&[0.4, 1.1, 3.9, 3.9, 1.1, 0.4]);
    assert_eq!(result, Mode::Trimodal(0.4, 1.1, 3.9));
}

#[test]
fn test_mode_floats_multimodal() {
    let result = mode(&[0.3, 1.7, 3.5, 3.5, 1.7, 0.3, 2.6, 2.6]);
    assert_eq!(result, Mode::Multimodal(vec![0.3, 1.7, 2.6, 3.5]));
}
