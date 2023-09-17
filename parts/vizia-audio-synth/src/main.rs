use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use vizia::prelude::*;
use crate::asdr::ADSR;
use crate::filters::Filter;
use crate::sampler::Sample;

mod oscillator;
mod asdr;
mod filters;
mod sampler;

static THEME: &'static str = include_str!("theme.css");
const TWOPI: f32 = 2.0 * 3.14159263;


// Messages to pass between gui and audio threads
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Message {
    Frequency(f32),
    Amplitude(f32),
    Note(f32),
}

// A controller widget which holds the knobs and the message channel
#[derive(Lens)]
struct AppData {
    command_sender: crossbeam_channel::Sender<Message>,
    amplitude: f32,
    frequency: f32,
}

pub enum AppEvent {
    SetAmplitude(f32),
    SetFrequency(f32),
}

impl Model for AppData {
    fn event(&mut self, _: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            AppEvent::SetAmplitude(amp) => {
                self.amplitude = *amp;
                self.command_sender
                    .send(Message::Amplitude(self.amplitude))
                    .unwrap();
            }

            AppEvent::SetFrequency(freq) => {
                self.frequency = *freq;
                self.command_sender
                    .send(Message::Frequency(self.frequency))
                    .unwrap();
            }
        });

        event.map(|window_event, _| match window_event {
            WindowEvent::KeyDown(code, _) if *code == Code::KeyZ => {
                self.command_sender.send(Message::Note(1.0)).unwrap();
            }

            WindowEvent::KeyUp(code, _) if *code == Code::KeyZ => {
                self.command_sender.send(Message::Note(0.0)).unwrap();
            }

            _ => {}
        })
    }
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

fn main() {
    // Create a channel for sending messages between threads
    let (command_sender, command_receiver) = crossbeam_channel::bounded(1024);

    // Move audio playback into another thread
    std::thread::spawn(move || {
        let host = cpal::default_host();

        let device = host
            .default_output_device()
            .expect("failed to find a default output device");

        let config = device.default_output_config().unwrap();


        match config.sample_format() {
            cpal::SampleFormat::F32 => {
                run::<f32>(&device, &config.into(), command_receiver.clone()).unwrap();
            }

            cpal::SampleFormat::I16 => {
                run::<i16>(&device, &config.into(), command_receiver.clone()).unwrap();
            }

            cpal::SampleFormat::U16 => {
                run::<u16>(&device, &config.into(), command_receiver.clone()).unwrap();
            }
        }
    });

    Application::new(move |cx| {
        cx.add_theme(THEME);

        AppData::new(command_sender.clone()).build(cx);

        HStack::new(cx, |cx| {
            VStack::new(cx, |cx| {
                Knob::new(cx, 0.5, AppData::amplitude, false)
                    .on_changing(|cx, val| cx.emit(AppEvent::SetAmplitude(val)));
                Label::new(cx, AppData::amplitude.map(|amp| format!("{:.2}", amp)));
            })
            .class("control");

            VStack::new(cx, |cx| {
                Knob::new(cx, 0.0, AppData::frequency, false)
                    .on_changing(|cx, val| cx.emit(AppEvent::SetFrequency(val)));
                Label::new(
                    cx,
                    AppData::frequency
                        .map(|freq| format!("{:.0} Hz", 440.0 + *freq * (2000.0 - 440.0))),
                );
            })
            .class("control");
        })
        .class("content");
    })
    .title("Audio Synth")
    .inner_size((200, 120))
    .run();
}

fn run<T>(
    device: &cpal::Device,
    config: &cpal::StreamConfig,
    command_receiver: crossbeam_channel::Receiver<Message>,
) -> Result<(), anyhow::Error>
where
    T: cpal::Sample,
{
    // Get the sample rate and channels number from the config
    let sample_rate = config.sample_rate.0 as f32;
    let channels = config.channels as usize;

    let err_fn = |err| eprintln!("an error occurred on stream: {}", err);

    // Define some variables we need for a simple oscillator
    let mut phi = 0.0f32;
    let mut frequency = 440.0f32;
    let mut amplitude = 0.1;
    let mut note_on = 0.0;
    let mut note_was_on = 1.0;

    let mut adsr_instance: ADSR = ADSR::new();
    let mut filter_instance: Filter = Filter::new();
    let mut sampler_instance: Sample = Sample::new();
    let _ = sampler_instance.load_sample("D:/Samples/Breaks/Amen-Break.wav");

    // Build an output stream
    let stream = device.build_output_stream(
        config,
        move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
            note_was_on = note_on;
            // A frame is a buffer of samples for all channels. So for 2 channels it's 2 samples.
            for frame in data.chunks_mut(channels) {
                // Try to receive a message from the gui thread
                while let Ok(command) = command_receiver.try_recv() {
                    match command {
                        Message::Note(val) => {
                            note_on = val;
                        }
                        Message::Amplitude(val) => {
                            amplitude = val;
                        }
                        Message::Frequency(val) => {
                            frequency = (val * (2000.0 - 440.0)) + 440.0;
                        }
                    }
                }
                if note_on != note_was_on {
                    adsr_instance.set_adsr(sample_rate);
                    filter_instance.set_filter(500.0, 1.0, sample_rate, filters::FilterType::LPF);
                    sampler_instance.set_sample(sample_rate);
                }
                // This creates a 'phase clock' which varies between 0.0 and 1.0 with a rate of frequency
                phi = (phi + (frequency / sample_rate)).fract();

                // Generate a sine wave signal
                // let adsr_value = ADSR1.gen_adsr(note as u8);
                let mut make_noise =
                    |phi: f32| -> f32 {
                        let result = amplitude * sampler_instance.gen_sample(note_on > 0.1, 0, sampler_instance.buffer_length, true, frequency);
                        result + filter_instance.gen_filter(
                        adsr_instance.gen_adsr(note_on != 0.0) *
                        0.1 * amplitude * (oscillator::oscillator(oscillator::OscillatorType::Square, phi * TWOPI)))
                    };


                // Convert the make_noise output into a sample
                let value: T = cpal::Sample::from::<f32>(&make_noise(phi));

                // Assign this sample to all channels in the frame
                for sample in frame.iter_mut() {
                    *sample = value;
                }
            }
        },
        err_fn,
    )?;

    // Play the stream
    stream.play()?;

    // Park the thread so out noise plays continuously until the app is closed
    std::thread::park();

    Ok(())
}

