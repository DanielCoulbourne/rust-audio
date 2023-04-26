use hound::WavWriter;
use std::fs::File;
use futures_core::stream::Stream;
use crate::audio::{AudioSample};

pub fn write_audio_stream_to_wav_file(
    filename: &str,
    audio_stream: impl Stream<Item = AudioSample>,
    sample_rate: u32,
    channels: u16,

) -> File {
    let mut wav_writer = WavWriter::new(
        File::create(filename).expect("to create file"),
        audio_stream.sample_rate,
        audio_stream.channels,
        audio_stream.bits_per_sample
    ).expect("to create wav writer");

    for sample in audio_stream.samples {
        wav_writer.write_sample(sample).expect("to write sample");
    }

    wav_writer.finalize().expect("to finalize wav writer")
}