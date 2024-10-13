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

func IsRealRoom(s string) (id int, real bool) {
	t := strings.Split(s, "-")
	idAndCheckSum, _ := lo.Last(t)
	name := strings.Join(lo.DropRight(t, 1), "")

	t2 := strings.Split(idAndCheckSum, "[")
	id, _ = strconv.Atoi(t2[0])
	checkSum := t2[1][:len(t2[1])-1]

	letterCounts := lo.Entries(lo.CountValues([]byte(name)))
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

func solve1(lines []string) int {
	return lo.Sum(
		lo.FilterMap(
			lines, func(l string, index int) (id int, real bool) {
				return IsRealRoom(l)
			}))
}

func main() {
	input := bufio.NewScanner(os.Stdin)
	lines := ReadAllLines(input)
	fmt.Println(solve1(lines))
}
