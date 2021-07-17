#[derive(Debug, PartialEq, Eq)]
pub enum Register {
	// Accumulator 8-bits register
	A,
	// Auxiliary 8-bits register
	B,
	C,
	D,
	E,
	F,
	H,
	L,
	// Program Counter 16-bits register
	PC,
	// Stack Pointer 16-bits register
	SP,
}
