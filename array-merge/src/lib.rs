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
mod test;
