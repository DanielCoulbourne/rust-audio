mod buffer;
// mod mp3;
mod async_mp3;
mod async_wav;
mod wav;
mod samples;
mod config;

#[allow(dead_code)]
mod audio;

mod mix;

use mix::mix;
use futures_core::stream::Stream;
use async_mp3::read_mp3_file_to_stream;
use async_stream::{ stream, try_stream };
use async_wav::write_audio_stream_to_wav_file;
use futures_util::StreamExt;
use std::pin::Pin;
use std::boxed::Box;

#[tokio::main]
async fn main() {
    let daniel = read_mp3_file_to_stream("stub-data/test1.mp3").await;
    let len = read_mp3_file_to_stream("stub-data/test2.mp3").await;

    let mut daniel_stream = Box::pin(daniel.fuse());
    let mut len_stream = Box::pin(len.fuse());

    let mixed_stream = stream! {
        loop {
            let daniel_sample = daniel_stream.next().await;
            let len_sample = len_stream.next().await;

            if daniel_sample.is_none() && len_sample.is_none() {
                break;
            }

            let mixed = daniel_sample.unwrap_or_default() + len_sample.unwrap_or_default();

            yield mixed;
        }
    };

    write_audio_stream_to_wav_file("test-output/mix.wav", mixed_stream, 44100, 2).await;

    /* let pinned_mixed_stream = Box::pin(mixed_stream); */
}
