package p2

import (
	"fmt"
	"testing"
)

type Matrix struct {
	a, b, c, d uint64
}

func (m *Matrix) multiply(o Matrix) {
	m.a, m.b, m.c, m.d =
		m.a*o.a+m.b*o.c,
		m.a*o.b+m.b*o.d,
		m.c*o.a+m.d*o.c,
		m.c*o.b+m.d*o.d
}

func identityMatrix() Matrix {
	return Matrix{a: 1, b: 0, c: 0, d: 1}
}

func cube() Matrix {
	i := identityMatrix()
	fib := Matrix{a: 1, b: 1, c: 1, d: 0}
	i.multiply(fib)
	i.multiply(fib)
	i.multiply(fib)
	return i
}

func Example() {
	i := identityMatrix()
	c := cube()
	sum := uint64(0)
	for i.b <= 4_000_000 {
		sum += i.b
		i.multiply(c)
	}
	fmt.Println(sum)
	// Output: 4613732
}

func TestExample(t *testing.T) {
	Example()
}

func BenchmarkExample(b *testing.B) {
	b.ResetTimer()
	for i := 0; i < b.N; i++ {
		Example()
	}
}
