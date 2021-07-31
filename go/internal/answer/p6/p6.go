package p6

import "fmt"

type Sequence struct {
	n uint64
}

func (s *Sequence) sumOfSquares() uint64 {
	return s.n * (s.n + 1) * (2*s.n + 1) / 6
}

func (s *Sequence) sum() uint64 {
	return (1 + s.n) * s.n / 2
}

func Example() {
	s := Sequence{n: 100}
	sum := s.sum()
	diff := sum*sum - s.sumOfSquares()
	fmt.Println(diff)
	// Output: 25164150
}
