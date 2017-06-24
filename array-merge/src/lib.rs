mod am {

    /// Merges two arrays togethers.
    /// The two arrays are:
    /// source[start; middle] and source[middle; end]
    /// The modified array is `destination`.
    /// `source` and `destination` should be the sames
    pub fn merge(
        source: &mut [u8],
        destination: &mut [u8],
        start: usize,
        end: usize,
        middle: usize,
    ) {
    }
}

#[cfg(test)]
mod tests {

    use am;

    #[test]
    fn test_two_items_sort() {

        let mut source = [5, 3];
        let mut destination = [5, 3];
        
        am::merge(
            &mut source,
            &mut destination,
            0,
            1,
            2,
        );

        assert_eq!(
            destination,
            [3, 5],
            "two items array is not sorted",
        );
    }
}
