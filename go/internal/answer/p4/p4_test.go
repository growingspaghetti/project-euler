package p4

import (
	"fmt"
	"testing"
)

func isPalindrome(p int) bool {
	t := int(p)
	q := 0
	for t > 0 {
		q = q*10 + t%10
		t /= 10
	}
	return p == q
}

func scanB(a, lpp int) int {
	for b := 999; b > a; b-- {
		p := a * b
		if p <= lpp {
			return lpp
		}
		if !isPalindrome(p) {
			continue
		}
		if p > lpp {
			lpp = p
		}
	}
	return lpp
}

func Example() {
	lpp := 0
	for a := 990; a >= 110; a -= 11 {
		lpp = scanB(a, lpp)
	}
	fmt.Println(lpp)
	// Output: 906609
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
