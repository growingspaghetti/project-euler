package answer

import "testing"

func TestBenchmarkSumOfAllMultiplesOfThreeOrFiveArithmeticSeries(t *testing.T) {
	got := sumOfAllMultiplesOfThreeOrFiveArithmeticSeries(1000)
	if got != 233168 {
		t.Errorf("SumOfAllMultiplesOfThreeOrFiveArithmeticSeries(1000) = %d; want 233168", got)
	}
}

func BenchmarkSumOfAllMultiplesOfThreeOrFiveArithmeticSeries(b *testing.B) {
	for n := 0; n < b.N; n++ {
		sumOfAllMultiplesOfThreeOrFiveArithmeticSeries(1000)
	}
}

// Running tool: /usr/local/go/bin/go test -benchmem -run=^$ github.com/growingspaghetti/project-euler/golang -bench ^(BenchmarkSumOfAllMultiplesOfThreeOrFiveArithmeticSeries)$ -v
//
// goos: linux
// goarch: amd64
// pkg: github.com/growingspaghetti/project-euler/golang
// BenchmarkSumOfAllMultiplesOfThreeOrFiveArithmeticSeries
// BenchmarkSumOfAllMultiplesOfThreeOrFiveArithmeticSeries-8   	126066117	         9.27 ns/op	       0 B/op	       0 allocs/op
// PASS
// ok  	github.com/growingspaghetti/project-euler/golang	2.136s
