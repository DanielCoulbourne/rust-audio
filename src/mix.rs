use crate::audio::AudioSample;

pub fn mix(samples: Vec<AudioSample>) -> AudioSample {
    let mut sum: i32 = 0;
    for sample in samples {
        sum += sample.sample as i32;
    }
    return AudioSample::new(sum as i16);
}