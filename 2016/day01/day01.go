package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

type Direction int

const (
	NORTH Direction = 0
	EAST  Direction = 1
	SOUTH Direction = 2
	WEST  Direction = 3
)

func (d *Direction) TurnRight() {
	*d = (*d + 1) % 4
}

func (d *Direction) TurnLeft() {
	*d = (*d + 3) % 4
}

type Position struct {
	x, y int
}

func (p *Position) Walk(direction Direction, distance int) {
	switch direction {
	case NORTH:
		p.y += distance
	case EAST:
		p.x -= distance
	case SOUTH:
		p.y -= distance
	case WEST:
		p.x += distance
	default:
		panic(fmt.Sprint("Invalid direction", direction))
	}
}

func Abs(v int) int {
	if v < 0 {
		return -v
	}
	return v
}

func solve1(line string) int {
	p := Position{0, 0}
	d := NORTH
	for _, s := range strings.Split(line, ", ") {
		switch s[0] {
		case 'L':
			d.TurnLeft()
		case 'R':
			d.TurnRight()
		default:
			panic(fmt.Sprint("invalid direction token:", s[0]))
		}
		steps, _ := strconv.Atoi(strings.TrimSpace(s[1:]))
		p.Walk(d, steps)
	}
	return Abs(p.x) + Abs(p.y)
}

func solve2(line string) int {
	p := Position{0, 0}
	d := NORTH
	visited := make(map[Position]bool)
	visited[p] = true
	for _, s := range strings.Split(line, ", ") {
		switch s[0] {
		case 'L':
			d.TurnLeft()
		case 'R':
			d.TurnRight()
		default:
			panic(fmt.Sprint("invalid direction token:", s[0]))
		}
		steps, _ := strconv.Atoi(strings.TrimSpace(s[1:]))
		for range steps {
			p.Walk(d, 1)
			_, v := visited[p]
			if v {
				return Abs(p.x) + Abs(p.y)
			}
			visited[p] = true
		}
	}
	panic("no solution -> no double visit")
}

func main() {
	input := bufio.NewReader(os.Stdin)
	line, _ := input.ReadString('\n')

	fmt.Println(solve1(line))
	fmt.Println(solve2(line))
}
