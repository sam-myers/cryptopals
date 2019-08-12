#[cfg(test)]
mod tests {
    use hamming::distance;

    #[test]
    fn test_hamming_distance_identical() {
        assert_eq!(
            distance(&[0b00000111, 0b00000110, 0b11111000], &[0b00000111, 0b00000110, 0b11111000]),
            0
        );
    }

    #[test]
    fn test_hamming_distance_example() {
        assert_eq!(
            distance("this is a test".as_bytes(), "wokka wokka!!!".as_bytes()),
            37
        );
    }
}
