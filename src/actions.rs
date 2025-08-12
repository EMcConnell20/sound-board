// -- Exports -- //

#[derive(Copy, Clone, Debug)]
pub enum Action<T> {
	/// Close the application
	Exit,
	/// Cut any playing audio and clear any audio queued up
	ClearAudio,
	/// Toggle output volume
	ToggleMute,
	/// Lower the output volume by this amount
	LowerVolume(f32),
	/// Increase the volume by this amount
	RaiseVolume(f32),
	/// Play this audio
	Play(T),
}
