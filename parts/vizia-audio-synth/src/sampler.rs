use std::f32::NEG_INFINITY;
use std::fs::File;
use std::io::{BufReader, Read};
use rodio::Source;
use std::cmp::max;

pub struct Sample {
    pub buffer: Vec<f32>,
    sr: f32,
    counter: f32,
    counter_delta: f32,
    pub buffer_length: u32,
    pub sample_has_ended: bool
}
impl Sample{
    pub fn new () -> Self {
        Sample {
            buffer: vec![],
            sr: 44100.0,
            buffer_length: 0,
            counter_delta: 1.0,
            counter: 0.0,
            sample_has_ended: false,
        }
    }
    pub fn load_sample(&mut self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let file_path = path;

        // Open the WAV file using a BufReader
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);

        // Create a Rodio decoder
        let source = rodio::Decoder::new(reader)?;
        self.sr = source.sample_rate() as f32;
        println!("{}", self.sr);

        // Read the audio data into a buffer
        self.buffer = source.convert_samples().collect();
        self.buffer_length = self.buffer.len() as u32;

        // Now the audio data is in the `buffer` vector.

        Ok(())
    }
    pub fn set_sample(&mut self, sample_rate: f32) {
        self.counter = 0.0;
        self.sample_has_ended = false;
        self.counter_delta = sample_rate/self.sr ;
        println!("{}", self.counter_delta);
    }
    pub fn gen_sample(&mut self, trigger: bool, init_position: u32, end_position: u32, is_loop: bool, speed: f32) -> f32 {
        let counter_delta = self.counter_delta * speed / 440.0;
        if end_position <= self.buffer_length && trigger{
            let mut result: f32 = 0.0;
            if self.counter < end_position as f32 {
                result = self.buffer[self.counter.round().min(self.buffer_length as f32 - 1.0) as usize];
                self.counter += counter_delta;
            }
            else if is_loop {
                result = self.buffer[0];
                self.counter = 0.0;
                self.counter += counter_delta;
            } else
            {
                self.sample_has_ended = true;
            }
            result

        } else {
            self.sample_has_ended = true;
            0.0
        }
    }
}
