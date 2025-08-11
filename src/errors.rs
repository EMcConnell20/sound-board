// -- Imports -- //

use thiserror::Error;

// -- Exports -- //

#[derive(Error, Debug)]
pub enum BoardError {
	#[error("The device \"{0}\" could not be found")]
	InvalidDeviceName(String),
}
