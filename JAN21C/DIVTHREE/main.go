package main

import (
	"fmt"
)

func getTotal(numbers []int) int {
	sum := 0
	for _, n := range numbers {
		sum += n
	}
	return sum
}

func calculateDiv3(total, problemsNeeded, days int) int {
	supportedDays := total / problemsNeeded
	if supportedDays < days {
		return supportedDays
	}
	return days
}

func main() {
	var n int
	fmt.Scan(&n)
	for i := 1; i <= n; i++ {
		var coders, problems, days int
		fmt.Scan(&coders, &problems, &days)
		totalProblems := make([]int, 0)
		for j := 1; j <= coders; j++ {
			var number int
			fmt.Scan(&number)
			totalProblems = append(totalProblems, number)
		}
		fmt.Println(
			calculateDiv3(getTotal(totalProblems), problems, days),
		)
	}
}
