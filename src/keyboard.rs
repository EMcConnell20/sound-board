// -- Imports -- //

use rdev::{EventType, Key};

use std::time::{Duration, Instant};
use std::sync::{LazyLock, Mutex};
use std::sync::atomic::{Ordering, AtomicBool};

// -- Exports -- //

// DOCS Control combos should only start with a `Mark`
// DOCS Play audio combos should not start with a `Mark`
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum KeyInput {
	/// `/` Special Input [Slash or KP_Divide]
	Mark = 0,
	/// `↑` Up Input [Up Arrow or KP_8]
	Up = 1,
	/// `↓` Down Input [Down Arrow or KP_2]
	Down = 2,
	/// `←` Left Input [Left Arrow or KP_4]
	Left = 3,
	/// `→` Right Input [Right Arrow or KP_6]
	Right = 4,
}

// DOCS Only one KeyboardWatcher should ever need to exist at a time.
#[derive(Debug)]
pub struct KeyboardWatcher<T> { input_tree: InputNode<T> }

// -- Constants -- //

const CHECK_IF_LISTENER_ACTIVE_INTERVAL: Duration = Duration::from_millis(50);
const RESET_INPUT_SEQUENCE_TIMER_LENGTH: Duration = Duration::from_secs(3);

// -- Statics -- //

static CURRENT_INPUT_SEQUENCE: LazyLock<Mutex<Vec<KeyInput>>> = LazyLock::new(
	|| Mutex::new(Vec::new())
);
static LAST_TIME_INPUT_WAS_RECORDED: LazyLock<Mutex<Instant>> = LazyLock::new(
	|| Mutex::new(Instant::now())
);

static LISTENER_ACTIVE: AtomicBool = AtomicBool::new(false);
static LISTENER_THREAD: LazyLock<std::thread::Thread> = LazyLock::new(
	|| std::thread::spawn(|| rdev::listen(input_listener)).thread().clone()
);

fn input_listener(e: rdev::Event) {
	let kdi = match e.event_type {
		EventType::KeyPress(Key::UpArrow | Key::Kp8) => KeyInput::Up,
		EventType::KeyPress(Key::DownArrow | Key::Kp2) => KeyInput::Down,
		EventType::KeyPress(Key::LeftArrow | Key::Kp4) => KeyInput::Left,
		EventType::KeyPress(Key::RightArrow | Key::Kp6) => KeyInput::Right,
		EventType::KeyPress(Key::Slash | Key::KpDivide) => KeyInput::Mark,
		
		// TODO Add a keybind that clears the sequence.
		// TODO Add a keybind to toggle input recording.
		
		EventType::KeyPress(Key::Return | Key::KpReturn) => {
			LISTENER_ACTIVE.store(false, Ordering::Release); // NOTE Does this need higher ordering?
			std::thread::park(); // NOTE Need to make sure this doesn't have unintended consequences.
			*LAST_TIME_INPUT_WAS_RECORDED.lock().unwrap() = Instant::now();
			return;
		}
		
		_ => return,
	};
	
	let mut sequence = CURRENT_INPUT_SEQUENCE.lock().unwrap();
	let mut earlier = LAST_TIME_INPUT_WAS_RECORDED.lock().unwrap();
	let now = Instant::now();
	
	// NOTE Consider adding a maximum sequence and checking for that here.
	if now.duration_since(*earlier) > RESET_INPUT_SEQUENCE_TIMER_LENGTH { sequence.clear() }
	*earlier = now;
	
	sequence.push(kdi);
}

// -- Export Impls -- //

impl <T> KeyboardWatcher<T> {
	// NOTE There should really only be one KeyboardWatcher at a time,
	//		so this function probably needs to guarantee that. A singleton
	//		approach would probably be okay, but it might be better to
	//		use a static bool and have this just return an Option<Self>.
	// TODO Make this guarantee that only one KeyboardWatcher is ever active.
	#[inline]
	pub const fn new() -> Self { Self { input_tree: InputNode::new() } }
	
	#[inline]
	pub fn insert(&mut self, t: T, sequence: impl IntoIterator<Item=KeyInput>) {
		self.input_tree.insert(t, sequence.into_iter());
	}
	
	#[inline]
	pub fn remove(&mut self, sequence: impl IntoIterator<Item=KeyInput>) {
		self.input_tree.remove(sequence.into_iter());
	}
	
	#[inline]
	pub fn get(&self, sequence: impl IntoIterator<Item=KeyInput>) -> Option<&T> {
		self.input_tree.get(sequence.into_iter())
	}
	
	#[inline]
	pub fn get_mut(&mut self, sequence: impl IntoIterator<Item=KeyInput>) -> Option<&mut T> {
		self.input_tree.get_mut(sequence.into_iter())
	}
	
	// NOTE Maybe make a version of this function that can timeout?
	pub fn listen_for_combo(&mut self) -> Vec<KeyInput> {
		LISTENER_ACTIVE.store(true, Ordering::Release);
		LISTENER_THREAD.unpark();
		
		loop {
			if !LISTENER_ACTIVE.load(Ordering::Acquire) { break }
			std::thread::sleep(CHECK_IF_LISTENER_ACTIVE_INTERVAL);
		};
		
		let mut out = Vec::new();
		out.append(&mut CURRENT_INPUT_SEQUENCE.lock().unwrap());
		
		out
	}
}

// -- Combo Nodes -- //

// NOTE There are probably better data structures for this, but this works well enough as it is.
#[derive(Debug)]
struct InputNode<T> { item: Option<T>, links: [Option<Box<Self>>; 5] }

impl <T> InputNode<T> {
	#[inline]
	const fn new() -> Self { Self { item: None, links: [None, None, None, None, None] } }
	
	fn from_iter(t: T, mut iter: impl Iterator<Item= KeyInput>) -> Self {
		let mut out = Self::new();
		
		if let Some(kdi) = iter.next() {
			out.links[kdi as usize] = Some(Box::new(Self::from_iter(t, iter)));
		} else {
			out.item = Some(t);
		};
		
		out
	}
	
	fn insert(&mut self, t: T, mut iter: impl Iterator<Item= KeyInput>) {
		let Some(kdi) = iter.next()
		else { self.item = Some(t); return };
		
		match &mut self.links[kdi as usize] {
			Some(ct) => ct.insert(t, iter),
			None => self.links[kdi as usize] = Some(Box::new(Self::from_iter(t, iter))),
		}
	}
	
	/// Returns
	/// - **true**: The parent node should remove this link.
	/// - **false**: The parent node should not remove this link.
	fn remove(&mut self, mut iter: impl Iterator<Item= KeyInput>) -> bool {
		let Some(kdi) = iter.next()
		else {
			self.item.take();
			return if let [None, None, None, None, None] = self.links { true } else { false }
		};
		
		let Some(ref mut ct) = self.links[kdi as usize]
		else { return false };
		
		if ct.remove(iter) {
			self.links[kdi as usize] = None;
			
			if let None = self.item && let [None, None, None, None, None] = self.links { true } else { false }
		} else { false }
	}
	
	fn get(&self, mut iter: impl Iterator<Item= KeyInput>) -> Option<&T> {
		let Some(kdi) = iter.next()
		else { return (&self.item).as_ref() };
		
		let Some(ref ct) = self.links[kdi as usize]
		else { return None };
		
		ct.get(iter)
	}
	
	fn get_mut(&mut self, mut iter: impl Iterator<Item= KeyInput>) -> Option<&mut T> {
		let Some(kdi) = iter.next()
		else { return (&mut self.item).as_mut() };
		
		let Some(ref mut ct) = self.links[kdi as usize]
		else { return None };
		
		ct.get_mut(iter)
	}
}
