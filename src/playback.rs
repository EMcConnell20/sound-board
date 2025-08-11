// -- Imports -- //

use crate::errors::BoardError;

use cpal::traits::{DeviceTrait, HostTrait};

// -- Exports -- //

pub struct Player {
	host: cpal::Host,
	stream: rodio::OutputStream,
	sink: rodio::Sink,
}

impl Player {
	pub fn new(audio_device_name: &str) -> Result<Self, Box<dyn std::error::Error>> {
		// Get the default host (CoreAudio on macOS)
		let host = cpal::default_host();
		
		// Get the specified audio device
		let Some(device) = host
			.devices()?
			.find(|device| device.name().is_ok_and(|s| &s == audio_device_name))
		else {
			return Err(BoardError::InvalidDeviceName(audio_device_name.to_string()).into())
		};
		
		let stream = rodio::OutputStreamBuilder::from_device(device)?.open_stream()?;
		let sink = rodio::Sink::connect_new(&stream.mixer());
		
		Ok(Self { host, stream, sink })
	}
	
	pub fn get_sink(&self) -> &rodio::Sink { &self.sink }
	
	pub fn play(&self) { self.sink.play() }
	pub fn pause(&self) { self.sink.pause() }
	pub fn clear(&self) { self.sink.clear() }
	
	pub fn is_paused(&self) -> bool { self.sink.is_paused() }
	pub fn is_empty(&self) -> bool { self.sink.empty() }
	
	pub fn play_audio<S>(&self, source: S) where
		S: rodio::Source + Send + 'static,
		f32: cpal::FromSample<S::Item>
	{
		self.sink.clear();
		self.sink.append(source);
		self.sink.play();
	}
}
