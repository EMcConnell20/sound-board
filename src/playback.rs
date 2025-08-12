// -- Imports -- //

use crate::errors::PlaybackError;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

// -- Consts -- //

const LOG_ON_DROP: bool = false;

// -- Exports -- //

pub struct Player {
	_host: cpal::Host,
	stream: rodio::OutputStream,
	sink: rodio::Sink,
}

impl Player {
	pub fn new(audio_device_name: &str) -> Result<Self, Box<dyn std::error::Error>> {
		// Get the default host (CoreAudio on macOS)
		let host = cpal::default_host();
		
		// Get the digital audio device
		let Some(device) = host.devices()?.find(|device| device.name().is_ok_and(|s| &s == audio_device_name))
		else { return Err(PlaybackError::InvalidDeviceName(audio_device_name.to_string()).into()) };
		
		if !device.supports_output() {
			return Err(PlaybackError::DeviceLacksOutput(audio_device_name.to_string()).into())
		}
		
		let mut stream = rodio::OutputStreamBuilder::from_device(device)?.open_stream()?;
		let sink = rodio::Sink::connect_new(&stream.mixer());
		
		stream.log_on_drop(LOG_ON_DROP);
		
		Ok(Self { _host: host, stream, sink })
	}
	
	pub fn is_paused(&self) -> bool { self.sink.is_paused() }
	pub fn is_empty(&self) -> bool { self.sink.empty() }
	
	pub fn get_volume(&self) -> f32 { self.sink.volume() }
	pub fn set_volume(&self, value: f32) { self.sink.set_volume(value) }
	pub fn lower_volume(&self, amount: &f32) {
		let vol = self.sink.volume();
		if vol <= *amount { self.sink.set_volume(0.0) }
		else { self.sink.set_volume(vol - amount) }
	}
	pub fn raise_volume(&self, amount: &f32) {
		let vol = self.sink.volume() + amount;
		if vol >= 1.5 { self.sink.set_volume(1.5) }
		else { self.sink.set_volume(vol) }
	}
	
	pub fn set_log_on_drop(&mut self, enabled: bool) { self.stream.log_on_drop(enabled) }
	
	pub fn play(&self) { self.sink.play() }
	pub fn pause(&self) { self.sink.pause() }
	pub fn clear(&self) { self.sink.clear() }
	
	pub fn play_audio<S>(&self, source: S) where
		S: rodio::Source + Send + 'static,
		f32: cpal::FromSample<S::Item>
	{
		self.sink.clear();
		self.sink.append(source);
		self.sink.play();
	}
}
