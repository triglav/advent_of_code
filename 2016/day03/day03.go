package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

func ReadAllLines(s *bufio.Scanner) []string {
	var lines []string
	for s.Scan() {
		lines = append(lines, s.Text())
	}
	return lines
}

func order(a, b *int) {
	if *b < *a {
		*a, *b = *b, *a
	}
}

func validateTriangle(e1, e2, e3 int) bool {
	order(&e1, &e2)
	order(&e1, &e3)
	order(&e2, &e3)
	return e1+e2 > e3
}

func solve1(lines []string) int {
	r := 0
	for _, line := range lines {
		f := strings.Fields(line)
		if len(f) != 3 {
			panic("unexpected input")
		}
		e1, _ := strconv.Atoi(f[0])
		e2, _ := strconv.Atoi(f[1])
		e3, _ := strconv.Atoi(f[2])
		if validateTriangle(e1, e2, e3) {
			r += 1
		}
	}
	return r
}

func main() {
	input := bufio.NewScanner(os.Stdin)
	lines := ReadAllLines(input)
	fmt.Println(solve1(lines))
}
