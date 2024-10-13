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

func Decompress(s string) string {
	var d strings.Builder
	for i := 0; i < len(s); {
		if s[i] == '(' {
			i++
			var marker []byte
			for s[i] != ')' {
				marker = append(marker, s[i])
				i++
			}
			i++
			t := strings.Split(string(marker), "x")
			if len(t) != 2 {
				panic(fmt.Sprint("unexpected marker", string(marker)))
			}
			charCount, _ := strconv.Atoi(t[0])
			repeatCount, _ := strconv.Atoi(t[1])
			characters := s[i : i+charCount]
			for range repeatCount {
				d.WriteString(characters)
			}
			i += charCount
			continue
		}
		d.WriteString(string(s[i]))
		i++
	}
	return d.String()
}

func solve1(lines []string) int {
	s := Decompress(lines[0])
	return len(s)
}

func main() {
	input := bufio.NewScanner(os.Stdin)
	lines := ReadAllLines(input)
	fmt.Println(solve1(lines))
}
