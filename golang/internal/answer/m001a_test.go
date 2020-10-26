package answer

import "testing"

func TestBenchmarkSumOfAllMultiplesOfThreeOrFive(t *testing.T) {
	got := sumOfAllMultiplesOfThreeOrFive(1000)
	if got != 233168 {
		t.Errorf("SumOfAllMultiplesOfThreeOrFive(1000) = %d; want 233168", got)
	}
}

func BenchmarkSumOfAllMultiplesOfThreeOrFive(b *testing.B) {
	for n := 0; n < b.N; n++ {
		sumOfAllMultiplesOfThreeOrFive(1000)
	}
}

// Running tool: /usr/local/go/bin/go test -benchmem -run=^$ github.com/growingspaghetti/project-euler/golang -bench ^(BenchmarkSumOfAllMultiplesOfThreeOrFive)$ -v
//
// goos: linux
// goarch: amd64
// pkg: github.com/growingspaghetti/project-euler/golang
// BenchmarkSumOfAllMultiplesOfThreeOrFive
// BenchmarkSumOfAllMultiplesOfThreeOrFive-8   	  512594	      2089 ns/op	       0 B/op	       0 allocs/op
// PASS
// ok  	github.com/growingspaghetti/project-euler/golang	1.100s
