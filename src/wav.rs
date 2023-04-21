use std::fs::remove_file;
use hound;

pub fn write_vector_to_wav_file(
    filename: &str,
    audio_buffer: Vec<i16>,
) {
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: 44100,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    let mut writer = hound::WavWriter::create(filename, spec).unwrap();
    
    for sample in audio_buffer {
        writer.write_sample(sample).unwrap();
    }

    writer.finalize().unwrap();
}

#[test]
fn write_vector_to_wav_file_test() {
    let mut audio_buffer = vec![];

    for _sample in 0..48_000 {
        audio_buffer.push(0 as i16);
    }

    write_vector_to_wav_file(
        "test-output/test.wav",
        audio_buffer
    );

    remove_file("test-output/test.wav").expect("to remove file");
}