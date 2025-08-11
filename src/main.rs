// -- Modules -- //

mod errors;
mod playback;
mod keyboard;
mod actions;
// -- Imports -- //

use playback::Player;
use keyboard::{KeyboardWatcher, KeyDirInput};

// -- Main -- //

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let mut kw: KeyboardWatcher<()> = KeyboardWatcher::new();
	kw.insert((), T_COMBO);
	
	loop {
		let sequence = kw.listen_for_combo();
		if let Some(_) = kw.get(sequence) { break }
	}
	
	println!("Correct Combo!");
	
	let player = Player::new("BlackHole 16ch")?;
	
	let src = rodio::Decoder::new_mp3(
		std::fs::File::open("media/audio/samples/fireball.mp3")?
	)?;
	
	player.play_audio(src);
	
	player.get_sink().sleep_until_end();
	
	Ok(())
}

// -- Testing -- //

const T_COMBO: [KeyDirInput; 4] = [KeyDirInput::Up, KeyDirInput::Down, KeyDirInput::Left, KeyDirInput::Right];
