use vizia::prelude::*;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Message {
    Note(f32),
    Frequency(f32),
    Amplitude(f32),
}

#[derive(Lens)]
struct AppData {
    command_sender: crossbeam_channel::Sender<Message>,
    amplitude: f32,
    frequency: f32,
}

impl AppData {
    pub fn new(command_sender: crossbeam_channel::Sender<Message>) -> Self {
        Self {
            command_sender,
            amplitude: 0.1,
            frequency: 0.0,
        }
    }
}

fn run<T>(device: &cpal::Device, config: &cpal::StreamConfig) -> Result<(), anyhow::Error>
where
    T: cpal::Sample,
{

    // Get the sample rate and channels number from the config
    let sample_rate = config.sample_rate.0 as f32;
    let channels = config.channels as usize;

    let err_fn = |err| eprintln!("an error occurred on stream: {}", err);

    // Define some variables we need for a simple oscillator
    let mut phi = 0.0f32;
    let mut frequency = 440.0;
    let mut amplitude = 0.1;
    let mut note = 1.0;

    // Build an output stream
    let stream = device.build_output_stream(
        config,
        move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
            for frame in data.chunks_mut(channels) {

                // This creates a 'phase clock' which varies between 0.0 and 1.0 with a rate of frequency
                phi = (phi + (frequency / sample_rate)).fract();

                let make_noise = |phi: f32| -> f32 {amplitude * (2.0 * std::f32::consts::PI * phi).sin()};

                // Convert the make_noise output into a sample
                let value: T = cpal::Sample::from::<f32>(&make_noise(phi));

                for sample in frame.iter_mut() {
                    *sample = value;
                }
            }
        },
        err_fn,
    )?;

    // Play the stream
    stream.play()?;

    // Park the thread so our noise plays continuously until the app is closed
    std::thread::park();

    Ok(())
}

fn main() {
    let (command_sender, command_receiver) = crossbeam_channel::bounded(1024);
    //UI/////////////////////////////////////////////////////////
    Application::new(|cx|{
        // UI will go here
    })
    .title("Audio Synth")
    .inner_size((200, 120))
    .run();



    //SYNTH/////////////////////////////////////////////////////
    std::thread::spawn(move || {

    let host = cpal::default_host();

    let device = host
        .default_output_device()
        .expect("failed to find a default output device");

    let config = device.default_output_config().unwrap();

    match config.sample_format() {
        cpal::SampleFormat::F32 => {
            run::<f32>(&device, &config.into()).unwrap();
        }

        cpal::SampleFormat::I16 => {
            run::<i16>(&device, &config.into()).unwrap();
        }

        cpal::SampleFormat::U16 => {
            run::<u16>(&device, &config.into()).unwrap();
        }

        _ => println!("Error, unknown sample format")
    }
});
}
