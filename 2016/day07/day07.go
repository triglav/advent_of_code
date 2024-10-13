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

func splitSequences(s string) (supernet, hypernet []string) {
	t1 := strings.Split(s, "[")

	supernet = []string{t1[0]}
	hypernet = []string{}

	for _, t := range t1[1:] {
		t2 := strings.Split(t, "]")
		if len(t2) != 2 {
			panic(fmt.Sprintf("unexpected token length: %d\n%s", len(t2), t2))
		}
		hypernet = append(hypernet, t2[0])
		supernet = append(supernet, t2[1])
	}
	return
}

func SupportsTLS(s string) bool {
	supernet, hypernet := splitSequences(s)
	for _, h := range hypernet {
		if HasABBA(h) {
			return false
		}
	}
	for _, h := range supernet {
		if HasABBA(h) {
			return true
		}
	}
	return false
}

func FindABA(supernet []string) []string {
	aba := []string{}
	for _, s := range supernet {
		for i := 0; i < len(s)-2; i++ {
			if s[i+0] != s[i+1] && s[i+0] == s[i+2] {
				aba = append(aba, s[i+0:i+3])
			}
		}
	}
	return aba
}

func ABA2BAB(aba string) string {
	bab := []byte{aba[1], aba[0], aba[1]}
	return string(bab)
}

func HasBAB(hypernet []string, bab string) bool {
	for _, s := range hypernet {
		for i := 0; i < len(s)-2; i++ {
			if s[i+0] == bab[0] && s[i+1] == bab[1] && s[i+2] == bab[2] {
				return true
			}
		}
	}
	return false
}

func SupportsSSL(s string) bool {
	supernet, hypernet := splitSequences(s)
	abas := FindABA(supernet)
	for _, aba := range abas {
		bab := ABA2BAB(aba)
		if HasBAB(hypernet, bab) {
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

func solve2(lines []string) int {
	count := 0
	for _, l := range lines {
		if SupportsSSL(l) {
			count += 1
		}
	}
	return count
}

func main() {
	input := bufio.NewScanner(os.Stdin)
	lines := ReadAllLines(input)
	fmt.Println(solve1(lines))
	fmt.Println(solve2(lines))
}
