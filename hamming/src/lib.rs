pub fn hamming_distance(left_seq : &str, right_seq : &str) -> Result<i32, i32> {
    let mut counter = 0;
    let mut index   = 0;

    if left_seq.len() == right_seq.len() {
        let left_bytes  = left_seq.as_bytes();
        let right_bytes = right_seq.as_bytes();

        for left_byte in left_bytes {
            if left_byte != right_bytes.get(index).unwrap() {
                counter += 1;
            }

            index += 1;
        }

        Ok(counter)
    } else {
        Err(-1)
    }
}
