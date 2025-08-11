// -- Imports -- //

use rdev::{EventType, Key};

use std::time::{Duration, Instant};
use std::sync::{LazyLock, Mutex};
use std::sync::atomic::{Ordering, AtomicBool};

// -- Exports -- //

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum KeyDirInput {
	/// Slash, KP_Divide
	Mark = 0,
	/// Up_Arrow, KP_8
	Up = 1,
	/// Down_Arrow, KP_2
	Down = 2,
	/// LeftArrow, KP_4
	Left = 3,
	/// RightArrow, KP_6
	Right = 4,
}

#[derive(Debug)]
pub struct KeyboardWatcher<T> { combos: ComboNode<T> }

// -- Constants -- //

const LISTEN_ACCESS_INTERVAL: Duration = Duration::from_millis(50);
const INPUT_RESET_TIMER: Duration = Duration::from_secs(3);

// -- Statics -- //

static COMBO_IS_ACTIVE: AtomicBool = AtomicBool::new(true);
static COMBO_SEQUENCE: LazyLock<Mutex<Vec<KeyDirInput>>> = LazyLock::new(|| Mutex::new(Vec::new()));
static LAST_COMBO_INPUT: LazyLock<Mutex<Instant>> = LazyLock::new(
	|| Mutex::new(Instant::now())
);
static LISTENER_THREAD: LazyLock<std::thread::Thread> = LazyLock::new(
	|| std::thread::spawn(|| rdev::listen(input_listener)).thread().clone()
);

fn input_listener(e: rdev::Event) {
	let kdi = match e.event_type {
		EventType::KeyPress(Key::UpArrow | Key::Kp8) => KeyDirInput::Up,
		EventType::KeyPress(Key::DownArrow | Key::Kp2) => KeyDirInput::Down,
		EventType::KeyPress(Key::LeftArrow | Key::Kp4) => KeyDirInput::Left,
		EventType::KeyPress(Key::RightArrow | Key::Kp6) => KeyDirInput::Right,
		EventType::KeyPress(Key::Slash | Key::KpDivide) => KeyDirInput::Mark,
		
		EventType::KeyPress(Key::Return | Key::KpReturn) => {
			COMBO_IS_ACTIVE.store(false, Ordering::Release);
			std::thread::park();
			*LAST_COMBO_INPUT.lock().unwrap() = Instant::now();
			return;
		}
		
		_ => return,
	};
	
	let mut seq = COMBO_SEQUENCE.lock().unwrap();
	let mut earlier = LAST_COMBO_INPUT.lock().unwrap();
	let now = Instant::now();
	
	if now.duration_since(*earlier) > INPUT_RESET_TIMER { seq.clear() }
	*earlier = now;
	
	seq.push(kdi);
}

// -- Export Impls -- //

impl <T> KeyboardWatcher<T> {
	#[inline]
	pub const fn new() -> Self { Self { combos: ComboNode::new() } }
	
	#[inline]
	pub fn insert(&mut self, t: T, sequence: impl IntoIterator<Item=KeyDirInput>) {
		self.combos.insert(t, sequence.into_iter());
	}
	
	#[inline]
	pub fn remove(&mut self, sequence: impl IntoIterator<Item=KeyDirInput>) {
		self.combos.remove(sequence.into_iter());
	}
	
	#[inline]
	pub fn get(&self, sequence: impl IntoIterator<Item=KeyDirInput>) -> Option<&T> {
		self.combos.get(sequence.into_iter())
	}
	
	#[inline]
	pub fn get_mut(&mut self, sequence: impl IntoIterator<Item=KeyDirInput>) -> Option<&mut T> {
		self.combos.get_mut(sequence.into_iter())
	}
	
	pub fn listen_for_combo(&mut self) -> Vec<KeyDirInput> {
		COMBO_IS_ACTIVE.store(true, Ordering::Release);
		LISTENER_THREAD.unpark();
		
		loop {
			if !COMBO_IS_ACTIVE.load(Ordering::Acquire) { break }
			std::thread::sleep(LISTEN_ACCESS_INTERVAL);
		};
		
		let mut out = Vec::new();
		out.append(&mut COMBO_SEQUENCE.try_lock().unwrap());
		
		out
	}
}

// -- Combo Nodes -- //


#[derive(Debug)]
struct ComboNode<T> { item: Option<T>, links: [Option<Box<Self>>; 5] }

impl <T> ComboNode<T> {
	#[inline]
	const fn new() -> Self { Self { item: None, links: [None, None, None, None, None] } }
	
	fn from(t: T, mut iter: impl Iterator<Item=KeyDirInput>) -> Self {
		let mut out = Self::new();
		
		if let Some(kdi) = iter.next() {
			out.links[kdi as usize] = Some(Box::new(Self::from(t, iter)));
		} else {
			out.item = Some(t);
		};
		
		out
	}
	
	fn insert(&mut self, t: T, mut iter: impl Iterator<Item=KeyDirInput>) {
		let Some(kdi) = iter.next()
		else { self.item = Some(t); return };
		
		match &mut self.links[kdi as usize] {
			Some(ct) => ct.insert(t, iter),
			None => self.links[kdi as usize] = Some(Box::new(Self::from(t, iter))),
		}
	}
	
	/// Returns
	/// - **true**: The parent node should remove this link.
	/// - **false**: The parent node should not remove this link.
	fn remove(&mut self, mut iter: impl Iterator<Item=KeyDirInput>) -> bool {
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
	
	fn get(&self, mut iter: impl Iterator<Item=KeyDirInput>) -> Option<&T> {
		let Some(kdi) = iter.next()
		else { return (&self.item).as_ref() };
		
		let Some(ref ct) = self.links[kdi as usize]
		else { return None };
		
		ct.get(iter)
	}
	
	fn get_mut(&mut self, mut iter: impl Iterator<Item=KeyDirInput>) -> Option<&mut T> {
		let Some(kdi) = iter.next()
		else { return (&mut self.item).as_mut() };
		
		let Some(ref mut ct) = self.links[kdi as usize]
		else { return None };
		
		ct.get_mut(iter)
	}
}
