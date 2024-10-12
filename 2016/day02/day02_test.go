package main

import "testing"

func TestSolve1(t *testing.T) {
	testCases := []struct {
		want string
		in   []string
	}{
		{
			in: []string{
				"ULL",
				"RRDDD",
				"LURDL",
				"UUUUD",
			},
			want: "1985",
		},
	}
	for _, tC := range testCases {
		t.Run("solve1", func(t *testing.T) {
			got := solve1(tC.in)
			if got != tC.want {
				t.Errorf("solve1() == %s, want %s", got, tC.want)
			}
		})
	}
}

func TestSolve2(t *testing.T) {
	testCases := []struct {
		want string
		in   []string
	}{
		{
			in: []string{
				"ULL",
				"RRDDD",
				"LURDL",
				"UUUUD",
			},
			want: "5DB3",
		},
	}
	for _, tC := range testCases {
		t.Run("solve2", func(t *testing.T) {
			got := solve2(tC.in)
			if got != tC.want {
				t.Errorf("solve2() == %s, want %s", got, tC.want)
			}
		})
	}
}
