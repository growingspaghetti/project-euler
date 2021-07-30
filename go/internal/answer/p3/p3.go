package p3

import (
	"fmt"
	"log"
	"math"
)

func divideFully(n *uint64, d uint64, side *uint64) {
	if *n%d != 0 {
		return
	}
	for ok := true; ok; ok = *n%d == 0 {
		*n /= d
	}
	*side = uint64(math.Sqrt(float64(*n)))
}

type Divisor struct {
	d uint64
	f uint8
}

func (d *Divisor) increment() {
	d.d += uint64(2) << d.f
	d.f ^= 1
}

func largestPrimeFactor(n uint64) uint64 {
	if n < 2 {
		log.Fatal("n must be > 1.")
	}
	side := uint64(math.Sqrt(float64(n)))
	for _, d := range []uint64{2, 3, 5} {
		divideFully(&n, d, &side)
		if n == 1 {
			return d
		}
	}
	d := Divisor{d: 5}
	for {
		d.increment()
		if d.d > side {
			break
		}
		divideFully(&n, d.d, &side)
		if n == 1 {
			return d.d
		}
	}
	return n
}

func Example() {
	ans := largestPrimeFactor(600851475143)
	fmt.Println(ans)
	// Output: 6857
}
