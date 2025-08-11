// -- Modules -- //

mod errors;
mod playback;
mod keyboard;
mod actions;

// -- Imports -- //

use actions::Action;
use playback::Player;
use keyboard::{KeyboardWatcher, KeyDirInput};

use rodio::Source;

// -- Macros -- //

macro_rules! combo { ($($tt:tt),+ $(,)?) => { [$(KeyDirInput::$tt),+] } }

macro_rules! insert_combo {
    ($nme:ident $com:ident $act:ident $(($num:literal))? $($fun:ident $str:literal)?) => {
		{
			$nme.insert(
				Action::$act$(($num))?$((rodio::Decoder::$fun(std::fs::File::open($str)?)?.buffered()))?,
				$com
			)
		}
	};
}

// -- Main -- //

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let player = Player::new("BlackHole 16ch")?;
	let mut kw: KeyboardWatcher<Action<_>> = KeyboardWatcher::new();
	
	insert_combo!(kw TOGGLE_MUTE_COMBO ToggleMute);
	insert_combo!(kw CLEAR_AUDIO_COMBO ClearAudio);
	insert_combo!(kw DECR_VOLUME_COMBO LowerVolume(0.25));
	insert_combo!(kw INCR_VOLUME_COMBO RaiseVolume(0.25));
	insert_combo!(kw DOUBLE_DECR_VOLUME_COMBO LowerVolume(0.5));
	insert_combo!(kw DOUBLE_INCR_VOLUME_COMBO RaiseVolume(0.5));
	insert_combo!(kw QUIT_COMBO Quit);
	
	insert_combo!(kw SPONGE_FAIL_COMBO Play new_mp3 "media/audio/samples/spongebob-fail.mp3");
	insert_combo!(kw VINE_BOOM_COMBO Play new_mp3 "media/audio/samples/vine-boom.mp3");
	insert_combo!(kw BASS_DROP_COMBO Play new_mp3 "media/audio/samples/bass-drop.mp3");
	insert_combo!(kw TACO_BELL_COMBO Play new_mp3 "media/audio/samples/taco-bell-bong.mp3");
	insert_combo!(kw AIR_HORN_COMBO Play new_mp3 "media/audio/samples/airhorn.mp3");
	insert_combo!(kw BUZZER_COMBO Play new_mp3 "media/audio/samples/buzzer.mp3");
	insert_combo!(kw YODA_FALL_COMBO Play new_mp3 "media/audio/samples/yoda-screaming.mp3");
	insert_combo!(kw LEGO_BREAK_COMBO Play new_mp3 "media/audio/samples/lego-break.mp3");
	insert_combo!(kw PROWLER_COMBO Play new_mp3 "media/audio/samples/prowler.mp3");
	insert_combo!(kw VANISH_COMBO Play new_mp3 "media/audio/samples/vanish.mp3");
	insert_combo!(kw FIREBALL_COMBO Play new_mp3 "media/audio/samples/fireball.mp3");
	insert_combo!(kw SPIDERMAN_COMBO Play new_mp3 "media/audio/samples/spiderman-reveal.mp3");
	
	loop {
		let sequence = kw.listen_for_combo();
		let Some(act) = kw.get(sequence) else { continue };
		
		match act {
			Action::Quit =>
				break,
			
			Action::ClearAudio =>
				player.clear(),
			
			Action::ToggleMute =>
				if player.get_volume() == 0.0 { player.set_volume(1.0) }
				else { player.set_volume(0.0) },
			
			Action::LowerVolume(i) =>
				player.lower_volume(i),
			
			Action::RaiseVolume(i) =>
				player.raise_volume(i),
			
			Action::Play(src) =>
				player.play_audio(src.clone()),
		}
	}
	
	Ok(())
}

// -- Combos -- //

// Controls
const TOGGLE_MUTE_COMBO: [KeyDirInput; 1] = combo![Mark];
const CLEAR_AUDIO_COMBO: [KeyDirInput; 2] = combo![Mark, Mark];
const DECR_VOLUME_COMBO: [KeyDirInput; 2] = combo![Mark, Down];
const INCR_VOLUME_COMBO: [KeyDirInput; 2] = combo![Mark, Up];
const DOUBLE_DECR_VOLUME_COMBO: [KeyDirInput; 3] = combo![Mark, Down, Down];
const DOUBLE_INCR_VOLUME_COMBO: [KeyDirInput; 3] = combo![Mark, Up, Up];
const QUIT_COMBO: [KeyDirInput; 6] = combo![Mark, Up, Right, Down, Down, Down];

// Sound Combos
const SPONGE_FAIL_COMBO: [KeyDirInput; 2] = combo![Left, Up];
const VINE_BOOM_COMBO: [KeyDirInput; 2] = combo![Left, Right];
const BASS_DROP_COMBO: [KeyDirInput; 2] = combo![Left, Down];

const TACO_BELL_COMBO: [KeyDirInput; 2] = combo![Right, Up];
const AIR_HORN_COMBO: [KeyDirInput; 2] = combo![Right, Left];
const BUZZER_COMBO: [KeyDirInput; 2] = combo![Right, Down];

const YODA_FALL_COMBO: [KeyDirInput; 2] = combo![Down, Down];
const LEGO_BREAK_COMBO: [KeyDirInput; 2] = combo![Down, Up];
const PROWLER_COMBO: [KeyDirInput; 2] = combo![Down, Left];
const VANISH_COMBO: [KeyDirInput; 2] = combo![Down, Right];

const FIREBALL_COMBO: [KeyDirInput; 2] = combo![Up, Down];
const SPIDERMAN_COMBO: [KeyDirInput; 2] = combo![Up, Left];
