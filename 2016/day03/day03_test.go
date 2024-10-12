package main

import "testing"

func TestValidateTriangle(t *testing.T) {
	testCases := []struct {
		a, b, c int
		want    bool
	}{
		{
			a: 5, b: 10, c: 25,
			want: false,
		},
		{
			a: 15, b: 10, c: 25,
			want: false,
		},
		{
			a: 15, b: 11, c: 25,
			want: true,
		},
	}
	for _, tC := range testCases {
		t.Run("solve1", func(t *testing.T) {
			got := validateTriangle(tC.a, tC.b, tC.c)
			if got != tC.want {
				t.Errorf("validateTriangle(%d, %d, %d) == %t, want %t", tC.a, tC.b, tC.c, got, tC.want)
			}
		})
	}
}
