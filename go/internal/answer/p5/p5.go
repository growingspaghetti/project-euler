package p5

import (
	"fmt"
	"log"
)

func gcd(a, b uint64) uint64 {
	if a == 0 || b == 0 {
		log.Fatal("gcd(0) is undefined.")
	}
	for b != 0 {
		a, b = b, a%b
	}
	return a
}

func lcm(a, b uint64) uint64 {
	return a * b / gcd(a, b)
}

func Example() {
	acc := uint64(2)
	for n := uint64(3); n <= 20; n++ {
		acc = lcm(acc, n)
	}
	fmt.Println(acc)
	// Output: 232792560
}
