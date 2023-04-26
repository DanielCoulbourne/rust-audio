use pretty_assertions::{assert_eq};

use crate::audio::AudioBuffer;

pub fn resample(
    audio_buffer: AudioBuffer,
    input_sample_rate_in_hz: u32,
    output_sample_rate_in_hz: u32,
) -> AudioBuffer {
    if input_sample_rate_in_hz == output_sample_rate_in_hz {
        return audio_buffer;
    }

    let mut resampled: AudioBuffer = AudioBuffer::new();
    let mut index: f32 = 0.0;
    let sample_rate_ratio: f32 = (input_sample_rate_in_hz as i16 / output_sample_rate_in_hz as i16) as f32;

    return loop {
        if let Some(sample) = audio_buffer.samples.get(index.clone().floor() as usize) {
            resampled.push(*sample);
        } else {
            break resampled;
        }

        index += sample_rate_ratio;
    }
}


#[test]
fn downsample_test() {
    let buffer = AudioBuffer::from_i16_vec(vec![1, 2, 3, 4, 5, 6, 7, 8]);

    let downsampled = resample(
        buffer,
        8,
        4
    );

    assert_eq!(
        downsampled,
        AudioBuffer::from_i16_vec(vec![1, 3, 5, 7])
    );
}


#[test]
fn upsample_test() {
    let buffer = AudioBuffer::from_i16_vec(vec![1, 2, 3, 4]);

    let upsampled = resample(
        buffer,
        4,
        8
    );

    assert_eq!(
        upsampled,
        AudioBuffer::from_i16_vec(vec![1, 1, 2, 2, 3, 3, 4, 4])
    );
}