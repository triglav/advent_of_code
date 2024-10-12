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

func (p *Position) Move2(direction byte, keypad *[5][5]byte) {
	p2 := *p
	switch direction {
	case 'U':
		p2.y = Max(p.y-1, 0)
	case 'D':
		p2.y = Min(p.y+1, 4)
	case 'L':
		p2.x = Max(p.x-1, 0)
	case 'R':
		p2.x = Min(p.x+1, 4)
	default:
		panic(fmt.Sprint("unexpected direction: ", direction))
	}
	if keypad[p2.y][p2.x] != ' ' {
		*p = p2
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

func solve2(lines []string) string {
	keypad := [5][5]byte{
		{' ', ' ', '1', ' ', ' '},
		{' ', '2', '3', '4', ' '},
		{'5', '6', '7', '8', '9'},
		{' ', 'A', 'B', 'C', ' '},
		{' ', ' ', 'D', ' ', ' '},
	}

	p := Position{0, 2}
	var r string
	for _, line := range lines {
		for _, c := range line {
			p.Move2(byte(c), &keypad)
		}
		r = fmt.Sprintf("%s%c", r, keypad[p.y][p.x])
	}
	return r
}

func main() {
	input := bufio.NewScanner(os.Stdin)
	lines := ReadAllLines(input)
	fmt.Println(solve1(lines))
	fmt.Println(solve2(lines))
}
