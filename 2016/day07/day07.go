package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
)

func ReadAllLines(s *bufio.Scanner) []string {
	var lines []string
	for s.Scan() {
		lines = append(lines, s.Text())
	}
	return lines
}

func HasABBA(s string) bool {
	for i := 0; i < len(s)-3; i++ {
		if s[i+0] != s[i+1] && s[i+0] == s[i+3] && s[i+1] == s[i+2] {
			return true
		}
	}
	return false
}

func SupportsTLS(s string) bool {
	t1 := strings.Split(s, "[")

	regular := []string{t1[0]}
	hypernet := []string{}

	for _, t := range t1[1:] {
		t2 := strings.Split(t, "]")
		if len(t2) != 2 {
			panic(fmt.Sprintf("unexpected token length: %d\n%s", len(t2), t2))
		}
		hypernet = append(hypernet, t2[0])
		regular = append(regular, t2[1])
	}

	for _, h := range hypernet {
		if HasABBA(h) {
			return false
		}
	}
	for _, h := range regular {
		if HasABBA(h) {
			return true
		}
	}
	return false
}

func solve1(lines []string) int {
	count := 0
	for _, l := range lines {
		if SupportsTLS(l) {
			count += 1
		}
	}
	return count
}

func main() {
	input := bufio.NewScanner(os.Stdin)
	lines := ReadAllLines(input)
	fmt.Println(solve1(lines))
}
