// -- Modules -- //

mod errors;
mod playback;
mod keyboard;
mod actions;

// -- Imports -- //

use actions::Action;
use playback::Player;
use keyboard::{KeyboardWatcher, KeyInput};

use rodio::Source;

// -- Macros -- //

macro_rules! combo { ($($tt:tt),+ $(,)?) => { [$(KeyInput::$tt),+] } }

macro_rules! add_combo_to_watcher {
    ($watch:ident $combo:ident $act:ident $(($num:literal))? $($fun:ident $str:literal)?) => {
		$watch.insert(
			Action::$act$(($num))?$((rodio::Decoder::$fun(std::fs::File::open($str)?)?.buffered()))?,
			$combo
		);
	};
}

// -- Main -- //

fn main() -> Result<(), Box<dyn std::error::Error>> {
	// TODO Add configurable device setup.
	let audio_player = Player::init(PLAYBACK_DEVICE)?;
	let mut kw: KeyboardWatcher<Action<_>> = KeyboardWatcher::new();
	
	add_combo_to_watcher!(kw TOGGLE_MUTE_COMBO ToggleMute);
	add_combo_to_watcher!(kw CLEAR_AUDIO_COMBO ClearAudio);
	add_combo_to_watcher!(kw DECR_VOLUME_COMBO LowerVolume(0.25));
	add_combo_to_watcher!(kw INCR_VOLUME_COMBO RaiseVolume(0.25));
	add_combo_to_watcher!(kw DOUBLE_DECR_VOLUME_COMBO LowerVolume(0.5));
	add_combo_to_watcher!(kw DOUBLE_INCR_VOLUME_COMBO RaiseVolume(0.5));
	add_combo_to_watcher!(kw EXIT_APP_COMBO Exit);
	
	add_combo_to_watcher!(kw SPONGE_FAIL_COMBO Play new_mp3 "media/audio/samples/spongebob-fail.mp3");
	add_combo_to_watcher!(kw VINE_BOOM_COMBO Play new_mp3 "media/audio/samples/vine-boom.mp3");
	add_combo_to_watcher!(kw BASS_DROP_COMBO Play new_mp3 "media/audio/samples/bass-drop.mp3");
	add_combo_to_watcher!(kw TACO_BELL_COMBO Play new_mp3 "media/audio/samples/taco-bell-bong.mp3");
	add_combo_to_watcher!(kw AIR_HORN_COMBO Play new_mp3 "media/audio/samples/airhorn.mp3");
	add_combo_to_watcher!(kw BUZZER_COMBO Play new_mp3 "media/audio/samples/buzzer.mp3");
	add_combo_to_watcher!(kw YODA_FALL_COMBO Play new_mp3 "media/audio/samples/yoda-screaming.mp3");
	add_combo_to_watcher!(kw LEGO_BREAK_COMBO Play new_mp3 "media/audio/samples/lego-break.mp3");
	add_combo_to_watcher!(kw PROWLER_COMBO Play new_mp3 "media/audio/samples/prowler.mp3");
	add_combo_to_watcher!(kw VANISH_COMBO Play new_mp3 "media/audio/samples/vanish.mp3");
	add_combo_to_watcher!(kw FIREBALL_COMBO Play new_mp3 "media/audio/samples/fireball.mp3");
	add_combo_to_watcher!(kw SPIDERMAN_COMBO Play new_mp3 "media/audio/samples/spiderman-reveal.mp3");
	
	loop {
		let input_sequence = kw.listen_for_combo();
		let Some(act) = kw.get(input_sequence) else { continue };
		
		match act {
			Action::Exit =>
				break,
			
			Action::ClearAudio =>
				audio_player.clear(),
			
			Action::ToggleMute =>
				if audio_player.get_volume() == 0.0 { audio_player.set_volume(1.0) }
				else { audio_player.set_volume(0.0) },
			
			Action::LowerVolume(i) =>
				audio_player.lower_volume(i),
			
			Action::RaiseVolume(i) =>
				audio_player.raise_volume(i),
			
			Action::Play(src) =>
				audio_player.play_audio(src.clone()),
		}
	}
	
	Ok(())
}

// -- Constants -- //

// Control Input Combos
const TOGGLE_MUTE_COMBO: [KeyInput; 1] = combo![Mark];
const CLEAR_AUDIO_COMBO: [KeyInput; 2] = combo![Mark, Mark];
const DECR_VOLUME_COMBO: [KeyInput; 2] = combo![Mark, Down];
const INCR_VOLUME_COMBO: [KeyInput; 2] = combo![Mark, Up];
const DOUBLE_DECR_VOLUME_COMBO: [KeyInput; 3] = combo![Mark, Down, Down];
const DOUBLE_INCR_VOLUME_COMBO: [KeyInput; 3] = combo![Mark, Up, Up];
const EXIT_APP_COMBO: [KeyInput; 6] = combo![Mark, Up, Right, Down, Down, Down];

// Sample Input Combos
const SPONGE_FAIL_COMBO: [KeyInput; 2] = combo![Left, Up];
const VINE_BOOM_COMBO: [KeyInput; 2] = combo![Left, Right];
const BASS_DROP_COMBO: [KeyInput; 2] = combo![Left, Down];
const TACO_BELL_COMBO: [KeyInput; 2] = combo![Right, Up];
const AIR_HORN_COMBO: [KeyInput; 2] = combo![Right, Left];
const BUZZER_COMBO: [KeyInput; 2] = combo![Right, Down];
const YODA_FALL_COMBO: [KeyInput; 2] = combo![Down, Down];
const LEGO_BREAK_COMBO: [KeyInput; 2] = combo![Down, Up];
const PROWLER_COMBO: [KeyInput; 2] = combo![Down, Left];
const VANISH_COMBO: [KeyInput; 2] = combo![Down, Right];
const FIREBALL_COMBO: [KeyInput; 2] = combo![Up, Down];
const SPIDERMAN_COMBO: [KeyInput; 2] = combo![Up, Left];

// TEMP Device Name
const PLAYBACK_DEVICE: &str = "BlackHole 16ch";
