package p1

import (
	"fmt"
	"testing"
)

func arithmeticSeries(n uint32) uint32 {
	d := 999 / n
	return n * d * (d + 1) / 2
}

func Example() {
	ans := arithmeticSeries(2) + arithmeticSeries(5) - arithmeticSeries(15)
	fmt.Println(ans)
	// Output: 233167
}

func TestExample(t *testing.T) {
	Example()
}
