mod buffer;
mod mp3;
mod wav;
mod samples;
mod config; 
mod audio;

use audio::AudioBuffer;
use mp3::{read_mp3_file_to_buffer, write_vector_to_mp3_file};
use buffer::mix;
use samples::resample;

fn main() {
    let daniel = read_mp3_file_to_buffer("stub-data/daniel.mp3");
    let caleb = read_mp3_file_to_buffer("stub-data/caleb.mp3");

    let mix: AudioBuffer = mix(&vec![daniel, caleb]);

    let resampled = resample(mix, 44100, 48000);

    write_vector_to_mp3_file("test-output/mix.mp3", resampled, 44100);
}