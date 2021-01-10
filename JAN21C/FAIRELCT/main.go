package main

import (
	"fmt"
	"sort"
)

func sumaArr(numbers []int) int {
	sum := 0
	for _, number := range numbers {
		sum += number
	}
	return sum
}
func main() {
	var n int
	fmt.Scan(&n)
	for ii := 1; ii <= n; ii++ {
		var can1, can2 int
		fmt.Scan(&can1, &can2)
		var can1Votes, can2Votes []int
		for j := 1; j <= can1; j++ {
			var votes int
			fmt.Scan(&votes)
			can1Votes = append(can1Votes, votes)
		}
		for j := 1; j <= can2; j++ {
			var votes int
			fmt.Scan(&votes)
			can2Votes = append(can2Votes, votes)
		}
		if sumaArr(can1Votes) > sumaArr(can2Votes) {
			fmt.Println(0)
			continue
		}
		sort.Ints(can1Votes)
		sort.Ints(can2Votes)
		for i, j := 0, len(can2Votes)-1; i < j; i, j = i+1, j-1 {
			can2Votes[i], can2Votes[j] = can2Votes[j], can2Votes[i]
		}
		min := len(can1Votes)
		if len(can2Votes) < min {
			min = len(can2Votes)
		}
		exist := false
		for j := 0; j < min; j++ {
			value := can1Votes[j]
			can1Votes[j] = can2Votes[j]
			can2Votes[j] = value
			if sumaArr(can1Votes) > sumaArr(can2Votes) {
				fmt.Println(j + 1)
				exist = true
				break
			}
		}
		if !exist {
			fmt.Println(-1)
		}
	}
}
