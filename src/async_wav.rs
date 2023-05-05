use hound::WavWriter;
use crate::audio::{AudioSample};

use futures_core::stream::Stream;
use futures_util::stream::StreamExt;

use tempfile::NamedTempFile;
use std::{io::Cursor, fs::File};
use futures::stream;
use std::f64::consts::PI;

pub async fn write_audio_stream_to_wav_file(
    filename: &str,
    audio_stream: impl Stream<Item = AudioSample> + Send + 'static,
    sample_rate: u32,
    channels: u16,
) -> () {
    let spec = hound::WavSpec {
        channels: channels,
        sample_rate: sample_rate,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    let mut wav_writer = WavWriter::create(filename, spec)
        .expect("to create wav writer");

    let mut pinned_audio_stream = Box::pin(audio_stream);

    while let Some(sample) = pinned_audio_stream.next().await {
        wav_writer.write_sample(sample.sample)
            .expect("to write audio sample");
    }

    wav_writer.finalize()
        .expect("to finalize wav writer");
}

#[tokio::test]
async fn test_write_audio_stream_to_wav_file() {
    // Stream that generates a simple sine wave
    let samples = (48000) as usize;

    let audio_stream = stream::iter((0..samples).map(
        |i| {
            let frequency = 440.0;
            let t = i as f64 / 48000 as f64;
            let value = (2.0 * PI * frequency * t).sin();
            let val = (value * i16::MAX as f64).round() as i16;

            AudioSample::new(val)
        }
    ));

    // Write the stream to the temporary file
    write_audio_stream_to_wav_file(
        "test-output/test.wav",
        audio_stream,
        48000,
        1,
    ).await;
}
