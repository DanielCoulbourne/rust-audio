use pretty_assertions::{assert_eq};

use std::{fs::{File, remove_file}, io::Write};
use mp3lame_encoder::{Builder, Id3Tag, DualPcm, FlushNoGap};
use minimp3::{Decoder, Frame, Error};

use crate::audio::{AudioSample, AudioBuffer};

pub fn read_mp3_file_to_iterator(
    filename: &str
) -> impl Iterator<Item = AudioSample> {
    let mut decoder = Decoder::new(
        File::open(filename).unwrap()
    );

    let mut output: AudioBuffer = AudioBuffer::new();

    loop {
        match decoder.next_frame() {
            Ok(Frame { data, sample_rate: _, channels, .. }) => {
                let mono: Vec<i16> = data.iter().step_by(channels).cloned().collect();
                output.samples.extend(
                    mono.iter().map(|sample| AudioSample {
                        sample: *sample
                    })
                );
            },
            Err(Error::Eof) => break output.samples.into_iter(),
            Err(e) => panic!("{:?}", e),
        }
    }
}

pub fn read_mp3_file_to_buffer(
    filename: &str
) -> AudioBuffer {
    let mut decoder = Decoder::new(
        File::open(filename).unwrap()
    );

    let mut output: AudioBuffer = AudioBuffer::new();

    loop {
        match decoder.next_frame() {
            Ok(Frame { data, sample_rate: _, channels, .. }) => {
                let mono: Vec<i16> = data.iter().step_by(channels).cloned().collect();
                output.samples.extend(
                    mono.iter().map(|sample| AudioSample {
                        sample: *sample
                    })
                );
            },
            Err(Error::Eof) => break output,
            Err(e) => panic!("{:?}", e),
        }
    }
}

pub fn write_buffer_to_mp3_file(
    filename: &str,
    audio_buffer: AudioBuffer,
    sample_rate: u32
) -> File {
    let mut mp3_enc_builder = Builder::new().expect("Create LAME builder");
    mp3_enc_builder.set_num_channels(2).expect("set channels");
    mp3_enc_builder.set_sample_rate(sample_rate).expect("set sample rate");
    mp3_enc_builder.set_brate(mp3lame_encoder::Birtate::Kbps256).expect("set brate");
    mp3_enc_builder.set_quality(mp3lame_encoder::Quality::Best).expect("set quality");

    mp3_enc_builder.set_id3_tag(Id3Tag {
        title: b"Test title",
        artist: &[],
        album: b"Test album",
        year: b"Current year",
        comment: b"Just my comment",
    });

    let mut mp3_encoder = mp3_enc_builder.build().expect("To initialize LAME encoder");

    let input = DualPcm {
        left: &audio_buffer.to_base(),
        right: &audio_buffer.to_base(),
    };

    let mut mp3_out_buffer = Vec::new();
    mp3_out_buffer.reserve(mp3lame_encoder::max_required_buffer_size(input.left.len()));
    let encoded_size = mp3_encoder.encode(input, mp3_out_buffer.spare_capacity_mut()).expect("To encode");
    
    unsafe {
        mp3_out_buffer.set_len(mp3_out_buffer.len().wrapping_add(encoded_size));
    }

    let encoded_size = mp3_encoder.flush::<FlushNoGap>(mp3_out_buffer.spare_capacity_mut()).expect("to flush");
    unsafe {
        mp3_out_buffer.set_len(mp3_out_buffer.len().wrapping_add(encoded_size));
    }

    let mut file = File::create(filename).expect("to create file");

    file.write_all(&mp3_out_buffer).expect("to write to file");

    file
}

#[test]
fn read_mp3_file_to_buffer_test() {
    let filename = "stub-data/silence128mono.mp3";

    let output: AudioBuffer = read_mp3_file_to_buffer(filename);

    // assert every sample is a 0
    assert_eq!(true, ! output.iter().any(|x| x != &AudioSample::new(0)));
}


#[test]
fn write_buffer_to_mp3_file_test() {
    let mut audio_buffer = AudioBuffer::new();

    for _sample in 0..48_000 {
        audio_buffer.push(
            AudioSample::new(0)
        );
    }

    let _file = write_buffer_to_mp3_file(
        "test-output/test.mp3",
        audio_buffer,
        48_000
    );

    let output = read_mp3_file_to_buffer("test-output/test.mp3");

    // assert every sample is a 0.0
    assert_eq!(true, ! output.iter().any(|x| x != &AudioSample::new(0)));

    remove_file("test-output/test.mp3").expect("to remove file");
}

#[test]
pub fn read_mp3_file_to_iterator_test() {
    let filename = "stub-data/silence128mono.mp3";

    let output: Vec<AudioSample> = read_mp3_file_to_iterator(filename).collect();

    // assert every sample is a 0
    assert_eq!(true, ! output.iter().any(|x| x != &AudioSample::new(0)));
}