
#[derive(Debug)]
pub enum Axis {
	LJSH = 1, // X
	LJSV = 2, // Y
	TRGL = 3, // Z
	
	RJSH = 4, // RX
	RJSV = 5, // RY
	TRGR = 6, // RZ
}

impl Axis {
	pub(super) fn to_evdev(&self) -> input_linux::AbsoluteAxis {
		use Axis::*;
		match &self {
			LJSH => input_linux::AbsoluteAxis::X,
			LJSV => input_linux::AbsoluteAxis::Y,
			TRGL => input_linux::AbsoluteAxis::Z,
			RJSH => input_linux::AbsoluteAxis::RX,
			RJSV => input_linux::AbsoluteAxis::RY,
			TRGR => input_linux::AbsoluteAxis::RZ,
		}
	}
	
	pub(super) fn all_axis() -> std::slice::Iter<'static, Self> {
		use Axis::*;
		[
			LJSH,
			LJSV,
			TRGL,
			RJSH,
			RJSV,
			TRGR,
		].iter()
	}
}