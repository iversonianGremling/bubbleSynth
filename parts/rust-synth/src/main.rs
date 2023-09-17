use cpal::{Sample, SampleFormat};
use cpal::traits::{HostTrait, DeviceTrait, StreamTrait};
use std::f32::consts::PI;
use std::time::Duration;


fn main()
{
    let host = cpal::default_host();
    let device = host.default_output_device().expect("no output device available");
    let mut supported_configs_range = device.supported_output_configs()
    .expect("error while querying configs");
    let supported_config = supported_configs_range.next()
    .expect("no supported config?!")
    .with_max_sample_rate();
    let err_fn = |err| eprintln!("an error occurred on the output audio stream: {}", err);
    let sample_format = supported_config.sample_format();
    let config = supported_config.into();
    let stream = match sample_format {
        SampleFormat::F32 => device.build_output_stream(&config, write_silence::<f32>, err_fn, None),
        SampleFormat::I16 => device.build_output_stream(&config, write_silence::<i16>, err_fn, None),
        SampleFormat::U16 => device.build_output_stream(&config, write_silence::<u16>, err_fn, None),
        sample_format => panic!("Unsupported sample format '{sample_format}'")
    }.unwrap();

    fn write_silence<F32: Sample>(data: &mut [f32], _: &cpal::OutputCallbackInfo) {
        let mut counter = 0;
        for sample in data.iter_mut() {
            counter += 1;
            *sample = f32::sin(2_f32*PI*440_f32*counter as f32);
        }
    }

    let activeTime = Duration::from_millis(1000);

    stream.play().unwrap();
    std::thread::sleep(activeTime);
    stream.pause().unwrap();
}

