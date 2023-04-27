
use minimp3::{Decoder, Error};

use tokio::fs::File;
use crate::audio::{AudioSample};
use futures_core::stream::Stream;

use async_stream::{stream};

use tokio_stream::StreamExt;
use tokio::runtime::Runtime;

pub async fn read_mp3_file_to_stream(
    filename: &str,
) -> impl Stream<Item = AudioSample> {
    let file = File::open(filename).await.unwrap();

    let mut decoder = Decoder::new(file);

    stream! {
        loop {
            match decoder.next_frame_future().await {
                Ok(frame) => {
                    for sample in frame.data {
                        yield AudioSample::new(sample);
                    }
                }
                Err(Error::Eof) => {
                    break
                },
                Err(e) => panic!("{:?}", e),
            }
        }
    }
}

#[test]
fn test_read_mp3_file_to_stream() {
    let filename = "stub-data/silence_mono.mp3";
    let rt = Runtime::new().unwrap();

    rt.block_on(async {
        let audio_samples_stream = read_mp3_file_to_stream(filename).await;

        let audio_samples = audio_samples_stream.collect::<Vec<AudioSample>>().await;

        assert!(!audio_samples.is_empty());

        let all_zeroes = audio_samples.iter().all(|sample| sample.sample == 0);
        assert!(all_zeroes);
    });
}
