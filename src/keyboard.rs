// -- Imports -- //

use std::clone::Clone;
use rdev::{Event, EventType, Key};

use std::sync::{LazyLock, Mutex};
use std::sync::atomic::AtomicBool;

// -- Exports -- //

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum KeyDirInput { Up, Down, Left, Right }

#[derive(Debug)]
pub struct ComboTree<T> {
	item: Option<T>,
	links: [Option<Box<Self>>; 4],
}

#[derive(Debug)]
pub struct KeyboardWatcher<T> {
	input_sequence: Vec<KeyDirInput>,
	combos: ComboTree<T>,
}

// -- Statics -- //

static LISTENER_THREAD: LazyLock<std::thread::Thread> = LazyLock::new(||
	std::thread::spawn(|| rdev::listen(input_listener)).thread().clone()
);
static COMBO_SEQUENCE: LazyLock<Mutex<Vec<KeyDirInput>>> = LazyLock::new(|| Mutex::new(Vec::new()));
static COMBO_IS_ACTIVE: AtomicBool = AtomicBool::new(true);

fn input_listener(e: Event) {
	let kdi = match e.event_type {
		EventType::KeyPress(Key::UpArrow) => KeyDirInput::Up,
		EventType::KeyPress(Key::DownArrow) => KeyDirInput::Down,
		EventType::KeyPress(Key::LeftArrow) => KeyDirInput::Left,
		EventType::KeyPress(Key::RightArrow) => KeyDirInput::Right,
		EventType::KeyPress(Key::Return | Key::KpReturn
		) => {
			COMBO_IS_ACTIVE.store(false, std::sync::atomic::Ordering::Release);
			std::thread::park();
			return;
		}
		
		_ => return,
	};
	
	COMBO_SEQUENCE.lock().unwrap().push(kdi);
}

// -- Impl -- //

impl <T> KeyboardWatcher<T> {
	pub fn new() -> Self { Self { input_sequence: Vec::new(), combos: ComboTree::new() } }
	
	pub fn insert(&mut self, t: T, sequence: impl IntoIterator<Item=KeyDirInput>) {
		self.combos.insert(t, sequence.into_iter());
	}
	
	pub fn remove(&mut self, sequence: impl IntoIterator<Item=KeyDirInput>) {
		self.combos.remove(sequence.into_iter());
	}
	
	pub fn listen_for_combo(&mut self) -> Vec<KeyDirInput> {
		COMBO_IS_ACTIVE.store(true, std::sync::atomic::Ordering::SeqCst);
		LISTENER_THREAD.unpark();
		
		loop {
			if !COMBO_IS_ACTIVE.load(std::sync::atomic::Ordering::Acquire) { break }
			std::thread::sleep(std::time::Duration::from_millis(10));
		};
		
		let mut out = Vec::new();
		out.append(&mut COMBO_SEQUENCE.try_lock().unwrap());
		
		out
	}
	
	pub fn get(&self, sequence: impl IntoIterator<Item=KeyDirInput>) -> Option<&T> {
		self.combos.get(sequence.into_iter())
	}
	
	pub fn get_mut(&mut self, sequence: impl IntoIterator<Item=KeyDirInput>) -> Option<&mut T> {
		self.combos.get_mut(sequence.into_iter())
	}
}

impl <T> ComboTree<T> {
	pub const fn new() -> Self { Self { item: None, links: [None, None, None, None] } }
	
	fn insert(&mut self, t: T, mut sequence: impl Iterator<Item=KeyDirInput>) {
		let Some(kdi) = sequence.next()
		else { self.item = Some(t); return };
		
		let idx = kdi.to_usize();
		
		match &mut self.links[idx] {
			Some(ct) => ct.insert(t, sequence),
			None => {
				let mut ct = Self::new();
				ct.insert(t, sequence);
				self.links[idx] = Some(Box::new(ct));
			}
		}
	}
	
	// Returns true if parent items can consider removing this link.
	fn remove(&mut self, mut sequence: impl Iterator<Item=KeyDirInput>) -> bool {
		let Some(kdi) = sequence.next()
		else {
			return if let Some(_) = self.item.take()
				&& let &[None, None, None, None] = &self.links { true } else { false }
		};
		
		let idx = kdi.to_usize();
		
		let Some(ref mut ct) = self.links[idx]
		else { return false };
		
		if ct.remove(sequence) {
			self.links[idx] = None;
			
			if let None = self.item
				&& let [None, None, None, None] = self.links { true } else { false }
		} else {
			false
		}
	}
	
	pub fn get(&self, mut sequence: impl Iterator<Item=KeyDirInput>) -> Option<&T> {
		let Some(kdi) = sequence.next()
		else { return (&self.item).as_ref() };
		
		let idx = kdi.to_usize();
		
		let Some(ref ct) = self.links[idx]
		else { return None };
		
		ct.get(sequence)
	}
	
	pub fn get_mut(&mut self, mut sequence: impl Iterator<Item=KeyDirInput>) -> Option<&mut T> {
		let Some(kdi) = sequence.next()
		else { return (&mut self.item).as_mut() };
		
		let idx = kdi.to_usize();
		
		let Some(ref mut ct) = self.links[idx]
		else { return None };
		
		ct.get_mut(sequence)
	}
}

impl KeyDirInput {
	pub const fn to_usize(&self) -> usize {
		match self {
			Self::Up => 0,
			Self::Down => 1,
			Self::Left => 2,
			Self::Right => 3,
		}
	}
}
