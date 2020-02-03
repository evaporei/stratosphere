pub fn mean(data: Vec<usize>) -> f32 {
    data.iter().fold(0, |acc, curr| acc + curr) as f32 / data.len() as f32
}

#[cfg(test)]
mod tests {
    use crate::mean;

    #[test]
    fn test_mean_positive_integers() {
        let result = mean(vec![0, 1, 2, 3, 4, 5]);
        assert_eq!(result, 2.5);
    }
}
