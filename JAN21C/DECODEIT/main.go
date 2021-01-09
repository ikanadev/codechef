package main

import "fmt"

var letters map[string]rune

func main() {
	letters = map[string]rune{
		"0000": 'a',
		"0001": 'b',
		"0010": 'c',
		"0011": 'd',
		"0100": 'e',
		"0101": 'f',
		"0110": 'g',
		"0111": 'h',
		"1000": 'i',
		"1001": 'j',
		"1010": 'k',
		"1011": 'l',
		"1100": 'm',
		"1101": 'n',
		"1110": 'o',
		"1111": 'p',
	}
	var cases int
	fmt.Scan(&cases)
	for i := 1; i <= cases; i++ {
		var charsQtty int
		var codes string
		chars := make([]rune, 0)
		fmt.Scan(&charsQtty)
		fmt.Scan(&codes)
		for j := 0; j < charsQtty/4; j++ {
			letterCode := codes[j*4 : j*4+4]
			letter := letters[letterCode]
			chars = append(chars, letter)
		}
		fmt.Printf("%s\n", string(chars))
	}
}
