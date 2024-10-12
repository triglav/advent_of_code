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
