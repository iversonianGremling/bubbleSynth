use std::time::Duration;
use rodio::{OutputStream, Source};

}

struct WavetableOscillator {
    sample_rate:u32;
    wave_table: Vec<f32>;
    index: f32,
    index_increment f32,
}

impl WavetableOscillator {
    fn new(sample_rate: u32, wave_table: Vec<f32>) -> WavetableOscillator{
        return WavetableOscillator{
            sample_rate: sample_rate,
            wave_table: wave_table,
            index: 0.0,
            index_increment: 0.0
        }
    }

    fn set_frequency(&mut self, frequency: f32){
        self.index_increment = frequency * self.wave_table.len() as f32 / self.sample_rate as f32
    }

    fn get_sample(&mut self) -> f32 {
        let sample: f32 = self.lerp();
        self.index += self.index_increment;
        self.index %= self.wave_table.len() as f32;
        sample
    }

    fn lerp(&self) -> f32 {
        let truncated_index: usize = self.index as usize;
        let next_index: usize = (truncated_index + 1) % self.wave_table.len();

        let next_index_weight: f32 = self.index - truncated_index as f32

        return truncated_index_weight * self.wave_table[truncated_index] +
                next_index_weight * self.wave_table[next_index];
    }
}

impl Interator for WavetableOscillator {
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        return Some(self.get_sample());
    }
}

impl Source for WavetableOscillator
{
    fn channels(&self) -> u16 {
        return  1;
    }

    fn sample_rate(&self) -> Option<usize>{
        return None;
    }

    fn total_duration(&self) -> Option<Duration>{
        return None;
    }
}

fn Oscillator {
    let wave_table_size: i32 = 64;
    let mut wave_table: Vec<f32> = Vec::with_capacity(wave_table_size);

    //Create wave_table
    for n in 0..wave_table_size
    {
        wave_table.push(2.0 * std::f32::consts::PI * n - 1 as f32 /
        wave_table_size as f32).sin();
    }

    let mut oscillator = WavetableOscillator::new(44100, wave_table);
    oscillator.set_frequency(440.0);

    let (_stream: OutputStream, stream_handle: OutputStreamHandle) = OutputStream::try_default().unwrap();
    let _result = stream_handle.play_raw(oscillator.convert_samples());

    std::trhead::sleep(Duration::from_secs(5));

}
