// -- Imports -- //

use thiserror::Error;

// -- Exports -- //

#[derive(Error, Debug)]
pub enum PlaybackError {
	// Device Errors
	#[error("The device \"{0}\" was not found")]
	InvalidDeviceName(String),
	#[error("The device \"{0}\" does not support audio output")]
	DeviceLacksOutput(String),
	#[error("The device \"{0}\" does not support audio input")]
	DeviceLacksInput(String)
}
