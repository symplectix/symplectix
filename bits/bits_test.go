package bits_test

import (
	"iter"
	"testing"

	"github.com/vec-jp/symplectix/bits"
)

var (
	_ bool = check[uint](0)
	_ bool = check[uint8](0)
	_ bool = check[uint16](0)
	_ bool = check[uint32](0)
	_ bool = check[uint64](0)
)

func check[T bits.Word](_ T) bool {
	return true
}

func upto(n int) iter.Seq[int] {
	seq := func(yield func(int) bool) {
		for i := range n {
			if !yield(i) {
				return
			}
		}
	}

	return seq
}

func TestRange(t *testing.T) {
	for i := range upto(10) {
		testRange(t, i)
	}
}

func FuzzRange(f *testing.F) {
	for i := range upto(10) {
		f.Add(i)
	}
	f.Fuzz(func(t *testing.T, i int) {
		testRange(t, i)
	})
}

func testRange(t *testing.T, i int) {
	if i >= 10 {
		t.Errorf("%d, %d", i, 10)
	}
}
