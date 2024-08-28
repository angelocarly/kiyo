/* 
By ytanimura
https://github.com/ytanimura/sound-shader/tree/main/src
*/

use cpal::traits::{DeviceTrait, HostTrait};
use cpal::{Device, Sample, SampleFormat, Stream, StreamConfig, SupportedStreamConfig};

pub struct StreamFactory {
	device: Device,
	config: SupportedStreamConfig,
}

impl StreamFactory {
	pub fn config(&self) -> StreamConfig {
		self.config.clone().into()
	}

	pub fn new(device: Device, config: SupportedStreamConfig) -> Self {
		Self { device, config }
	}

	pub fn default_factory() -> Result<StreamFactory, String> {
		let host = cpal::default_host();
		let device = host
			.default_output_device()
			.ok_or("failed to find output device")?;
		let config = device
			.default_output_config()
			.map_err(|e| format!("{}", e))?;
		if config.channels() != 2 {
			return Err(format!(
				"audio channel must be 2\nchannels: {}",
				config.channels()
			));
		}
		Ok(Self { device, config })
	}

	pub fn create_stream(
		&self,
		routin: impl FnMut(usize) -> Vec<f32> + Send + 'static,
	) -> Result<Stream, String> {
		match self.config.sample_format() {
			SampleFormat::F32 => self.sub_get_stream::<f32, _>(routin),
			SampleFormat::I16 => self.sub_get_stream::<i16, _>(routin),
			SampleFormat::U16 => self.sub_get_stream::<u16, _>(routin),
		}
	}
	fn sub_get_stream<T: Sample, F: FnMut(usize) -> Vec<f32> + Send + 'static>(
		&self,
		mut routin: F,
	) -> Result<Stream, String> {
		self.device
			.build_output_stream(
				&self.config.clone().into(),
				move |output: &mut [T], _: &cpal::OutputCallbackInfo| {
					routin(output.len())
						.into_iter()
						.zip(output)
						.for_each(|(b, a)| *a = cpal::Sample::from(&b))
				},
				|err| eprintln!("an error occurred on stream: {}", err),
			)
			.map_err(|e| format!("{}", e))
	}
}

#[test]
fn beep() {
	use cpal::traits::StreamTrait;
	let sf = StreamFactory::default_factory().unwrap();
	println!("{:?}", sf.config());
	let sample_rate = sf.config().sample_rate.0;
	let mut sample_clock = 0;
	let routin = move |len: usize| -> Vec<f32> {
		(0..len / 2)
			.flat_map(|_| {
				sample_clock = (sample_clock + 1) % sample_rate;
				let r = sample_clock as f32 / sample_rate as f32;
				let a = f32::sin(2.0 * std::f32::consts::PI * 440.0 * r);
				vec![a, a]
			})
			.collect()
	};
	let stream = sf.create_stream(routin).unwrap();
	stream.play().unwrap();
	std::thread::sleep(std::time::Duration::from_millis(1000));
}
