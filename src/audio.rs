use futures::Stream;

#[derive(Copy, Clone, Debug)]
pub struct AudioSample {
    pub sample: i16,
}

impl AudioSample {
    pub fn new(sample: i16) -> AudioSample {
        AudioSample { sample }
    }
}

impl Default for AudioSample {
    fn default() -> Self {
        AudioSample { sample: 0 }
    }
}

impl PartialEq for AudioSample {
    fn eq(&self, other: &Self) -> bool {
        self.sample == other.sample
    }
}

#[derive(Clone, Debug)]
pub struct AudioBuffer {
    pub samples: Vec<AudioSample>,
}

impl AudioBuffer {
    pub fn new() -> AudioBuffer {
        AudioBuffer { samples: Vec::new() }
    }

    pub fn push(&mut self, sample: AudioSample) {
        self.samples.push(sample);
    }

    pub fn iter(&self) -> std::slice::Iter<AudioSample> {
        self.samples.iter()
    }

    pub fn from_i16_vec(samples: Vec<i16>) -> AudioBuffer {
        let mut buffer = AudioBuffer::new();

        for sample in samples {
            buffer.push(AudioSample::new(sample));
        }

        buffer
    }

    pub fn to_base(&self) -> Vec<i16> {
        self.samples
            .iter()
            .map(|sample| sample.sample)
            .collect()
    }
}

impl PartialEq for AudioBuffer {
    fn eq(&self, other: &Self) -> bool {
        self.samples == other.samples
    }
}


struct AudioStream {}

// impl Stream for AudioStream {
//     type Item = AudioSample;

//     fn poll_next(self: std::pin::Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
//         todo!()
//     }
// }