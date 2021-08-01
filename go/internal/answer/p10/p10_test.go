package p10

import (
	"fmt"
	"math"
	"testing"
)

type Index struct {
	i int
	f uint8
}

func (i *Index) increment() {
	i.i += 2 << i.f
	i.f ^= 1
}

func ruleout(sieve []bool, p int) {
	for i := p * p; i < len(sieve); i += p {
		sieve[i] = true
	}
}

func Example() {
	n := 2_000_000
	sieve := make([]bool, n+1)
	side := int(math.Sqrt(float64(n)))
	i := Index{i: 5}
	sum := 2 + 3
	for i.i <= side {
		if !sieve[i.i] {
			ruleout(sieve, i.i)
			sum += i.i
		}
		i.increment()
	}
	for i.i < len(sieve) {
		if !sieve[i.i] {
			sum += i.i
		}
		i.increment()
	}

	fmt.Println(sum)
	// Output: 142913828922
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
