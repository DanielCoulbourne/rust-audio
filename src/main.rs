mod buffer;
// mod mp3;

#[cfg(test)]
mod async_mp3;

mod async_wav;
mod wav;
mod samples;
mod config;

#[allow(dead_code)] 
mod audio;

// use mp3::{read_mp3_file_to_iterator};
// use samples::{mix};

fn main() {
    // let host_audio = read_mp3_file_to_iterator("stub-data/test1.mp3");
    // let guest_audio = read_mp3_file_to_iterator("stub-data/test2.mp3");
    
    
    // let output_file = File::create("output.mp3").unwrap();

    // let tracks = vec![host_audio, guest_audio];
    // let loop_index = 0;

    // loop {
    //     let samples = tracks.map(|track| track.next().unwrap());

    //     let mixed_sample = mix(samples);
    // }
}