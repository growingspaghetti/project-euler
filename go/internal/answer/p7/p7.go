package p7

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

func ruleout_from_position(sieve []bool, prime, position int) {
	pos := (((position - 1) / prime) + 1) * prime
	sq := prime * prime
	if pos < sq {
		pos = sq
	}
	for i := pos; i < len(sieve); i += prime {
		sieve[i] = true
	}
}

func extend(sieve *[]bool, primes []int) {
	previousLen := len(*sieve)
	*sieve = append(*sieve, make([]bool, previousLen)...)
	for _, p := range primes {
		ruleout_from_position(*sieve, p, previousLen)
	}
}

func Example() {
	counter := 2
	sieve := make([]bool, 10000)
	primes := []int{}
	i := Index{i: 5}
Exploration:
	for {
		for i.i < len(sieve) {
			if sieve[i.i] {
				i.increment()
				continue
			}
			counter++
			if counter == 10001 {
				break Exploration
			}
			primes = append(primes, i.i)
			ruleout(sieve, i.i)
			i.increment()
		}
		extend(&sieve, primes)
	}
	//fmt.Println(i.i)
	// Output: 104743
}
