package answer

func sumOfAllMultiplesOfThreeOrFive(below int) int {
	sum := 0
	for i := 0; i < below; i++ {
		if i%3 == 0 || i%5 == 0 {
			sum += i
		}
	}
	return sum
}
