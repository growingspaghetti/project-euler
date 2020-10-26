package answer

func arithmeticSeries(first, last, n int) int {
	doubleOfAverage := first + last
	return doubleOfAverage * n / 2
}

func sumOfAllMultiplesOfThreeOrFiveArithmeticSeries(below int) int {
	equalOrLessThan := below - 1
	a3 := arithmeticSeries(0, equalOrLessThan-equalOrLessThan%3, equalOrLessThan/3+1)
	a5 := arithmeticSeries(0, equalOrLessThan-equalOrLessThan%5, equalOrLessThan/5+1)
	a15 := arithmeticSeries(0, equalOrLessThan-equalOrLessThan%15, equalOrLessThan/15+1)
	return a3 + a5 - a15
}
