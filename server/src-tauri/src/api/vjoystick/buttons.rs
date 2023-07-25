#[derive(Debug)]
pub enum Button {
	// Button Buttons
	A = 1,
	B = 2,
	X = 3,
	Y = 4,
	
	Select = 5,
	Start = 6,

	ShoulderL = 7,
	ShoulderR = 8,

	ThumbL = 9,
	ThumbR = 10,

	Mode = 11
}

impl Button {
	pub(super) fn to_evdev(&self) -> input_linux::Key {
		use Button::*;
		match &self {
			A => input_linux::Key::ButtonSouth,
			B => input_linux::Key::ButtonEast,
			X => input_linux::Key::ButtonNorth,
			Y => input_linux::Key::ButtonWest,
			Select => input_linux::Key::ButtonSelect,
			Start => input_linux::Key::ButtonStart,
			ShoulderL => input_linux::Key::ButtonTL,
			ShoulderR => input_linux::Key::ButtonTR,
			ThumbL => input_linux::Key::ButtonThumbl,
			ThumbR => input_linux::Key::ButtonThumbr,
			Mode => input_linux::Key::ButtonMode,
		}
	}
	
	pub(super) fn all_buttons() -> std::slice::Iter<'static, Self> {
		use Button::*;
		[
			A,
			B,
			X,
			Y,
			Select,
			Start,
			ShoulderL,
			ShoulderR,
			ThumbL,
			ThumbR,
			Mode
		].iter()
	}
}