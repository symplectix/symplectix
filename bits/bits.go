package bits

// Unsigned integers with a fixed-sized bits.
type Word interface {
	~uint | ~uint8 | ~uint16 | ~uint32 | ~uint64
}
