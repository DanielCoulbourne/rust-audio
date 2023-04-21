use pretty_assertions::{assert_eq};

use crate::audio::{AudioBuffer, AudioSample};

pub fn mix(
    buffers: &Vec<AudioBuffer>
) -> AudioBuffer {
    let mut index: i32 = 0;
    let mut mixed =  AudioBuffer::new();

    return loop {
        if ! buffers.iter().any(
                |buffer| buffer.samples.get(index as usize).is_some()
            )
        {
            break mixed;
        }

        let mut sum_of_buffer_values: i32 = 0;

        for buffer in buffers {
            if let Some(sample) = buffer.samples.get(index as usize) {
                sum_of_buffer_values += sample.sample as i32;
            }
        }

        mixed.push(
            AudioSample::new(sum_of_buffer_values as i16)
        );

        index += 1;
    }
}

#[test]
fn mix_test() {
    let buf1 = AudioBuffer::from_i16_vec(vec![2, 2, 2, 2, 2]);
    let buf2 = AudioBuffer::from_i16_vec(vec![1, 2, 3, 4, 5, 6, 7, 8]);

    let mixed = mix(&vec![buf1, buf2]);

    assert_eq!(mixed, AudioBuffer::from_i16_vec(vec![3, 4, 5, 6, 7, 6, 7, 8]));
}