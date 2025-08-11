// -- Exports -- //

#[derive(Copy, Clone, Debug)]
pub enum Action<T> {
	Quit,
	ClearAudio,
	ToggleMute,
	LowerVolume(f32),
	RaiseVolume(f32),
	Play(T),
}
