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

type HitMap map[byte]int

func (h *HitMap) Add(c byte) {
	_, has := (*h)[c]
	if has {
		(*h)[c] += 1
	} else {
		(*h)[c] = 1
	}
}

func (h *HitMap) MostCommon() byte {
	var mostCommon byte
	maxCount := 0
	for c, count := range *h {
		if count > maxCount {
			maxCount = count
			mostCommon = c
		}
	}
	return mostCommon
}

func solve1(lines []string) string {
	length := len(lines[0])
	h := make([]HitMap, length)
	for i := range h {
		h[i] = make(HitMap)
	}
	for _, l := range lines {
		for i, c := range l {
			h[i].Add(byte(c))
		}
	}
	s := []byte{}
	for i := range h {
		s = append(s, h[i].MostCommon())
	}
	return string(s)
}

func main() {
	input := bufio.NewScanner(os.Stdin)
	lines := ReadAllLines(input)
	fmt.Println(solve1(lines))
}
