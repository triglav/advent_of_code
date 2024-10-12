package main

import "testing"

func TestTurnRight(t *testing.T) {
	testCases := []struct {
		in, want Direction
	}{
		{in: NORTH, want: EAST},
		{in: EAST, want: SOUTH},
		{in: SOUTH, want: WEST},
		{in: WEST, want: NORTH},
	}
	for _, tC := range testCases {
		t.Run("TurnRight", func(t *testing.T) {
			got := tC.in
			got.TurnRight()
			if got != tC.want {
				t.Errorf("TurnRight(%d) == %d, want %d", tC.in, got, tC.want)
			}
		})
	}
}

func TestTurnLeft(t *testing.T) {
	testCases := []struct {
		in, want Direction
	}{
		{in: NORTH, want: WEST},
		{in: WEST, want: SOUTH},
		{in: SOUTH, want: EAST},
		{in: EAST, want: NORTH},
	}
	for _, tC := range testCases {
		t.Run("TurnLeft", func(t *testing.T) {
			got := tC.in
			got.TurnLeft()
			if got != tC.want {
				t.Errorf("TurnLeft(%d) == %d, want %d", tC.in, got, tC.want)
			}
		})
	}
}

func TestSolve1(t *testing.T) {
	testCases := []struct {
		in   string
		want int
	}{
		{
			in:   "R10, L10",
			want: 20,
		},
		{
			in:   "R2, L3",
			want: 5,
		},
		{
			in:   "R2, R2, R2",
			want: 2,
		},
		{
			in:   "R5, L5, R5, R3",
			want: 12,
		},
	}
	for _, tC := range testCases {
		t.Run(tC.in, func(t *testing.T) {
			got := solve1(tC.in)
			if got != tC.want {
				t.Errorf("TurnLeft(%s) == %d, want %d", tC.in, got, tC.want)
			}
		})
	}
}

func TestSolve2(t *testing.T) {
	testCases := []struct {
		in   string
		want int
	}{
		{
			in:   "R8, R4, R4, R8",
			want: 4,
		},
	}
	for _, tC := range testCases {
		t.Run(tC.in, func(t *testing.T) {
			got := solve2(tC.in)
			if got != tC.want {
				t.Errorf("TurnLeft(%s) == %d, want %d", tC.in, got, tC.want)
			}
		})
	}
}
