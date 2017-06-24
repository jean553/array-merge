mod am {

    /// Merges two arrays togethers.
    /// The two arrays are:
    /// source[start; middle] and source[middle; end]
    /// The destination array is `destination`.
    #[allow(dead_code)]
    pub fn merge(
        source: &mut [u8],
        destination: &mut [u8],
        start: usize,
        end: usize,
        middle: usize,
    ) {

        let mut i = start;
        let mut j = middle;

        for index in start..end {

            if (j >= end || source[i] <= source[j]) && i < middle {
                destination[index] = source[i];
                i += 1;
            }
            else
            {
                destination[index] = source[j];
                j += 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use am;

    #[test]
    fn test_four_items_sort() {

        let mut source = [3, 5, 2, 7];
        let mut destination = [0, 0, 0, 0];

        am::merge(
            &mut source,
            &mut destination,
            0,
            4,
            2,
        );

        assert_eq!(
            destination,
            [2, 3, 5, 7],
            "two items array is not sorted",
        );
    }

    #[test]
    fn test_six_items_sort() {

        let mut source = [8, 15, 23, 0, 17, 25];
        let mut destination = [0, 0, 0, 0, 0, 0];

        am::merge(
            &mut source,
            &mut destination,
            0,
            6,
            3,
        );

        assert_eq!(
            destination,
            [0, 8, 15, 17, 23, 25],
            "four items array is not sorted",
        );
    }

    #[test]
    fn test_ten_items_sort() {

        let mut source = [8, 15, 23, 188, 190, 0, 17, 25, 150, 170];
        let mut destination = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

        am::merge(
            &mut source,
            &mut destination,
            0,
            10,
            5,
        );

        assert_eq!(
            destination,
            [0, 8, 15, 17, 23, 25, 150, 170, 188, 190],
            "ten items array is not sorted",
        );
    }
}
