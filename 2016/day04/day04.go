package main

import (
	"bufio"
	"fmt"
	"os"
	"slices"
	"strconv"
	"strings"

	"github.com/samber/lo"
)

func ReadAllLines(s *bufio.Scanner) []string {
	var lines []string
	for s.Scan() {
		lines = append(lines, s.Text())
	}
	return lines
}

func parseRoom(s string) (name, checkSum string, id int) {
	t := strings.Split(s, "-")
	idAndCheckSum, _ := lo.Last(t)
	name = strings.Join(lo.DropRight(t, 1), "-")

	t2 := strings.Split(idAndCheckSum, "[")
	id, _ = strconv.Atoi(t2[0])
	checkSum = t2[1][:len(t2[1])-1]
	return
}

func IsRealRoom(s string) (id int, real bool) {
	name, checkSum, id := parseRoom(s)

	letterCounts := lo.Entries(
		lo.CountValues(
			lo.Filter([]byte(name),
				func(c byte, index int) bool { return c != '-' })))
	slices.SortFunc(letterCounts, func(a, b lo.Entry[byte, int]) int {
		if a.Value == b.Value {
			return int(a.Key) - int(b.Key)
		}
		return b.Value - a.Value
	})
	checkSum2 := strings.Join(
		lo.Map(letterCounts[0:5],
			func(e lo.Entry[byte, int], index int) string { return string(e.Key) }), "")
	real = checkSum == checkSum2
	return
}

func shiftLetter(c byte, count int) byte {
	// A becomes B, B becomes C, Z becomes A, and so on. Dashes become spaces.
	if c >= 'a' && c <= 'z' {
		return byte((int(c-'a')+count)%int('z'-'a'+1)) + 'a'
	}
	if c == '-' {
		return ' '
	}
	panic("unexpected letter")
}

func DecryptRoomName(s string) string {
	name, _, id := parseRoom(s)
	var s2 []byte
	for _, c := range name {
		s2 = append(s2, shiftLetter(byte(c), id))
	}
	return string(s2)
}

func solve1(lines []string) int {
	return lo.Sum(
		lo.FilterMap(
			lines, func(l string, index int) (id int, real bool) {
				return IsRealRoom(l)
			}))
}

func solve2(lines []string) int {
	for _, l := range lines {
		id, real := IsRealRoom(l)
		if !real {
			continue
		}
		name := DecryptRoomName(l)
		// use grep on all names, the following works with my dataset
		if strings.HasPrefix(name, "north") {
			return id
		}
	}
	panic("not sure")
}

func main() {
	input := bufio.NewScanner(os.Stdin)
	lines := ReadAllLines(input)
	fmt.Println(solve1(lines))
	fmt.Println(solve2(lines))
}
