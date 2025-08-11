// -- Modules -- //

mod errors;
mod playback;
mod keyboard;
mod actions;

// -- Imports -- //

use rodio::Source;
use playback::Player;
use actions::Action;
use keyboard::{KeyboardWatcher, KeyDirInput};

// -- Main -- //

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let player = Player::new("BlackHole 16ch")?;
	let src = rodio::Decoder::new_mp3(
		std::fs::File::open("media/audio/samples/fireball.mp3")?
	)?.buffered();
	
	
	let mut kw: KeyboardWatcher<Action<()>> = KeyboardWatcher::new();
	kw.insert(Action::Quit, QUIT_COMBO);
	kw.insert(Action::Mute, MUTE_COMBO)	;
	kw.insert(Action::Unmute, UNMUTE_COMBO);
	kw.insert(Action::Play(()), PLAY_COMBO);
	
	loop {
		let sequence = kw.listen_for_combo();
		let Some(act) = kw.get(sequence) else { continue };
		
		match act {
			Action::Quit => break,
			Action::Mute => player.get_sink().set_volume(0.0),
			Action::Unmute => player.get_sink().set_volume(1.0),
			Action::Play(()) => player.play_audio(src.clone()),
		}
	}
	
	Ok(())
}

// -- Testing -- //

const QUIT_COMBO: [KeyDirInput; 5] = [
	KeyDirInput::Up,
	KeyDirInput::Right,
	KeyDirInput::Down,
	KeyDirInput::Down,
	KeyDirInput::Down,
];

const MUTE_COMBO: [KeyDirInput; 2] = [KeyDirInput::Up, KeyDirInput::Up];
const UNMUTE_COMBO: [KeyDirInput; 2] = [KeyDirInput::Down, KeyDirInput::Down];

const PLAY_COMBO: [KeyDirInput; 2] = [KeyDirInput::Left, KeyDirInput::Right];
