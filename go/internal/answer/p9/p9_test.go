package p9

import (
	"fmt"
	"math"
	"testing"
)

func Example() {
	var ans uint32
Exploration:
	for m := uint32(2); m <= uint32(math.Sqrt(float64(499))); m++ {
		for n := uint32(1); n < m; n++ {
			a := m*m - n*n
			b := 2 * m * n
			c := m*m + n*n
			if a+b+c == 1000 {
				ans = a * b * c
				break Exploration
			}
		}
	}
	fmt.Println(ans)
	// Output: 31875000
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
