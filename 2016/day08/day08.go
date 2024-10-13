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

type Screen struct {
	pixels []bool
	w, h   int
}

func NewScreen(w, h int) Screen {
	return Screen{
		w:      w,
		h:      h,
		pixels: make([]bool, w*h),
	}
}

func (s Screen) String() string {
	out := []byte{}
	for y := range s.h {
		for x := range s.w {
			if s.Get(x, y) {
				out = append(out, '#')
			} else {
				out = append(out, '.')
			}
		}
		out = append(out, '\n')
	}
	return string(out)
}

func (s *Screen) pixel(x, y int) *bool {
	if x < 0 || x >= s.w {
		panic("Invalid 'x' coordinate")
	}
	if y < 0 || y >= s.h {
		panic("Invalid 'y' coordinate")
	}
	return &s.pixels[s.w*y+x]
}

func (s *Screen) Set(x, y int) {
	*s.pixel(x, y) = true
}

func (s *Screen) Clear(x, y int) {
	*s.pixel(x, y) = false
}

func (s *Screen) Get(x, y int) bool {
	return *s.pixel(x, y)
}

func (s *Screen) CountLitPixels() int {
	c := 0
	for _, p := range s.pixels {
		if p {
			c += 1
		}
	}
	return c
}

func (s *Screen) Rect(w, h int) {
	for y := range h {
		for x := range w {
			s.Set(x, y)
		}
	}
}

func (s *Screen) RotateColumn(x, c int) {
	column := []bool{}
	for y := 0; y < s.h; y++ {
		column = append(column, s.Get(x, y))
	}
	if len(column) != s.h {
		panic("invalid column size")
	}
	for y := 0; y < s.h; y++ {
		*s.pixel(x, (y+c)%s.h) = column[y]
	}
}

func (s *Screen) RotateRow(y, c int) {
	row := []bool{}
	for x := 0; x < s.w; x++ {
		row = append(row, s.Get(x, y))
	}
	if len(row) != s.w {
		panic("invalid row size")
	}
	for x := 0; x < s.w; x++ {
		*s.pixel((x+c)%s.w, y) = row[x]
	}
}

func (s *Screen) Execute(c string) {
	if strings.HasPrefix(c, "rect ") {
		c2 := strings.Split(c[5:], "x")
		if len(c2) != 2 {
			panic("invalid rect param")
		}
		w, _ := strconv.Atoi(c2[0])
		h, _ := strconv.Atoi(c2[1])
		s.Rect(w, h)
		return
	}
	if strings.HasPrefix(c, "rotate column x=") {
		c2 := strings.Split(c[16:], " by ")
		if len(c2) != 2 {
			panic("invalid rotate column param")
		}
		x, _ := strconv.Atoi(c2[0])
		c, _ := strconv.Atoi(c2[1])
		s.RotateColumn(x, c)
		return
	}
	if strings.HasPrefix(c, "rotate row y=") {
		c2 := strings.Split(c[13:], " by ")
		if len(c2) != 2 {
			panic("invalid rotate row param")
		}
		y, _ := strconv.Atoi(c2[0])
		c, _ := strconv.Atoi(c2[1])
		s.RotateRow(y, c)
		return
	}
	panic("unknown command")
}

func solve1(lines []string) int {
	s := NewScreen(50, 6)
	for _, l := range lines {
		s.Execute(l)
	}
	fmt.Println(s)
	return s.CountLitPixels()
}

func main() {
	input := bufio.NewScanner(os.Stdin)
	lines := ReadAllLines(input)
	fmt.Println(solve1(lines))
}
