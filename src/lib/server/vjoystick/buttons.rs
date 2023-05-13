pub enum Button {
	// Button Buttons
	A = 1,
	B = 2,
	X = 4,
	Y = 5,
	
	// Shoulder
	ShoulderLEFT = 7,
	ShoulderRIGHT = 8,
	
	// Joystick
	JsLEFT = 14,
	JsRIGHT = 15,
	
	// DPAD
	UP = 16,
	DOWN = 17,
	LEFT = 18,
	RIGHT = 19,
}

impl Button {
	pub(super) fn to_evdev(&self) -> input_linux::Key {
		use Button::*;
		match &self {
			A => input_linux::Key::ButtonSouth,
			B => input_linux::Key::ButtonEast,
			X => input_linux::Key::ButtonNorth,
			Y => input_linux::Key::ButtonWest,
			ShoulderLEFT => input_linux::Key::ButtonTL,
			ShoulderRIGHT => input_linux::Key::ButtonTR,
			JsLEFT => input_linux::Key::ButtonThumbl,
			JsRIGHT => input_linux::Key::ButtonThumbr,
			UP => input_linux::Key::ButtonDpadUp,
			DOWN => input_linux::Key::ButtonDpadDown,
			LEFT => input_linux::Key::ButtonDpadLeft,
			RIGHT => input_linux::Key::ButtonDpadRight,
		}
	}
	
	pub(super) fn all_buttons() -> std::slice::Iter<'static, Self> {
		use Button::*;
		[
			A,
			B,
			X,
			Y,
			ShoulderLEFT,
			ShoulderRIGHT,
			JsLEFT,
			JsRIGHT,
			UP,
			DOWN,
			LEFT,
			RIGHT
		].iter()
	}
}