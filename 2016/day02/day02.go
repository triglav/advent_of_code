package main

import (
	"bufio"
	"fmt"
	"os"
)

func ReadAllLines(s *bufio.Scanner) []string {
	var lines []string
	for s.Scan() {
		lines = append(lines, s.Text())
	}
	return lines
}

type Position struct {
	x, y int
}

func Min(a, b int) int {
	if a < b {
		return a
	}
	return b
}

func Max(a, b int) int {
	if a > b {
		return a
	}
	return b
}

func (p *Position) Move(direction byte) {
	switch direction {
	case 'U':
		p.y = Max(p.y-1, 0)
	case 'D':
		p.y = Min(p.y+1, 2)
	case 'L':
		p.x = Max(p.x-1, 0)
	case 'R':
		p.x = Min(p.x+1, 2)
	default:
		panic(fmt.Sprint("unexpected direction: ", direction))
	}
}

func solve1(lines []string) string {
	keypad := [3][3]byte{{'1', '2', '3'}, {'4', '5', '6'}, {'7', '8', '9'}}

	p := Position{1, 1}
	var r string
	for _, line := range lines {
		for _, c := range line {
			p.Move(byte(c))
		}
		r = fmt.Sprintf("%s%c", r, keypad[p.y][p.x])
	}
	return r
}

func main() {
	input := bufio.NewScanner(os.Stdin)
	lines := ReadAllLines(input)
	fmt.Println(solve1(lines))
}
