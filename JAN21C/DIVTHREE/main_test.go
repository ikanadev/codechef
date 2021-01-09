package main

import "testing"

func TestGetTotal(t *testing.T) {
	t.Run("It should return the correct value", func(t *testing.T) {
		cases := []struct {
			numbers []int
			result  int
		}{
			{
				numbers: []int{1, 2, 3, 4, 5},
				result:  15,
			},
			{
				numbers: []int{},
				result:  0,
			},
			{
				numbers: []int{8, 2, 3},
				result:  13,
			},
		}

		for _, b := range cases {
			got := getTotal(b.numbers)
			if got != b.result {
				t.Errorf("Expected %d, but got %d", b.result, got)
			}
		}
	})
}

func TestCalculateDiv3(t *testing.T) {
	t.Run("It should calculate how many days can there will be contest", func(t *testing.T) {
		cases := [][4]int{
			{4, 5, 31, 0},
			{23, 10, 3, 2},
			{56, 5, 7, 7},
			{21, 5, 10, 4},
			{3, 3, 300, 1},
		}

		for _, arr := range cases {
			got := calculateDiv3(arr[0], arr[1], arr[2])
			if got != arr[3] {
				t.Errorf("Expected %d, but got %d", arr[3], got)
			}
		}
	})
}
