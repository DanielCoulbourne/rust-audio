pub fn mix(samples: Vec<i16>) -> i16 {
    let mut sum: i32 = 0;
    for sample in samples {
        sum += sample as i32;
    }
    return sum as i16;
}