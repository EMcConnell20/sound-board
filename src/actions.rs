// -- Exports -- //

pub enum Action<T> {
	Quit,
	Mute,
	Unmute,
	Play(T),
}

// -- Implementations -- //

impl <T: std::fmt::Debug> std::fmt::Debug for Action<T> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Action::Quit => write!(f, "Quit"),
			Action::Mute => write!(f, "Mute"),
			Action::Unmute => write!(f, "Unmute"),
			Action::Play(t) => write!(f, "Play({t:?})"),
		}
	}
}

impl <T: std::fmt::Display> std::fmt::Display for Action<T> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Quit => write!(f, "Quit"),
			Self::Mute => write!(f, "Mute"),
			Self::Unmute => write!(f, "Unmute"),
			Self::Play(t) => write!(f, "Play: {t}"),
		}
	}
}

impl <T: Copy> Copy for Action<T> {}
impl <T: Clone> Clone for Action<T> { fn clone(&self) -> Self { self.clone() } }
