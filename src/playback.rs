// -- Imports -- //

use crate::errors::PlaybackError;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

// -- Consts -- //

// TEMP This should be controlled by a config/feature, not a constant.
const LOG_ON_DROP: bool = false;

// -- Exports -- //

// NOTE Does the _host field even need to exist?
pub struct Player {
	_host: cpal::Host,
	stream: rodio::OutputStream,
	sink: rodio::Sink,
}

impl Player {
	pub fn init(audio_device_name: &str) -> Result<Self, Box<dyn std::error::Error>> {
		// Get the default host (CoreAudio on macOS)
		let host = cpal::default_host();
		
		// Iterate through available devices and get the one with the name that was given.
		let Some(device) = host.devices()?.find(|device| device.name().is_ok_and(|s| &s == audio_device_name))
		else { return Err(PlaybackError::InvalidDeviceName(audio_device_name.to_string()).into()) };
		
		// TODO Make sure the device supports audio input as well.
		if !device.supports_output() {
			return Err(PlaybackError::DeviceLacksOutput(audio_device_name.to_string()).into())
		}
		
		let mut stream = rodio::OutputStreamBuilder::from_device(device)?.open_stream()?;
		let sink = rodio::Sink::connect_new(&stream.mixer());
		
		stream.log_on_drop(LOG_ON_DROP);
		
		Ok(Self { _host: host, stream, sink })
	}
	
	pub fn lower_volume(&self, amount: &f32) {
		let vol = self.sink.volume();
		if vol <= *amount { self.sink.set_volume(0.0) }
		else { self.sink.set_volume(vol - amount) }
	}
	
	pub fn raise_volume(&self, amount: &f32) {
		let vol = self.sink.volume() + amount;
		// TEMP Move the maximum volume limit to a constant or something.
		if vol >= 1.5 { self.sink.set_volume(1.5) }
		else { self.sink.set_volume(vol) }
	}
	
	pub fn play_audio<S>(&self, source: S) where
		S: rodio::Source + Send + 'static,
		f32: cpal::FromSample<S::Item>
	{
		self.sink.clear();
		self.sink.append(source);
		self.sink.play();
	}
	
	// TEMP Will add useful functionality to these later.
	
	#[inline]
	pub fn is_paused(&self) -> bool { self.sink.is_paused() }
	
	#[inline]
	pub fn is_empty(&self) -> bool { self.sink.empty() }
	
	#[inline]
	pub fn get_volume(&self) -> f32 { self.sink.volume() }
	
	#[inline]
	pub fn set_volume(&self, value: f32) { self.sink.set_volume(value) }
	
	#[inline]
	pub fn set_log_on_drop(&mut self, enabled: bool) { self.stream.log_on_drop(enabled) }
	
	#[inline]
	pub fn play(&self) { self.sink.play() }
	
	#[inline]
	pub fn pause(&self) { self.sink.pause() }
	
	#[inline]
	pub fn clear(&self) { self.sink.clear() }
}
