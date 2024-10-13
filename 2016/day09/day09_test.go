package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestExecute(t *testing.T) {
	testCases := []struct {
		in, want string
	}{
		{
			in:   "ADVENT",
			want: "ADVENT",
		},
		{
			in:   "A(1x5)BC",
			want: "ABBBBBC",
		},
		{
			in:   "(3x3)XYZ",
			want: "XYZXYZXYZ",
		},
		{
			in:   "A(2x2)BCD(2x2)EFG",
			want: "ABCBCDEFEFG",
		},
		{
			in:   "(6x1)(1x3)A",
			want: "(1x3)A",
		},
		{
			in:   "X(8x2)(3x3)ABCY",
			want: "X(3x3)ABC(3x3)ABCY",
		},
	}
	for _, tC := range testCases {
		t.Run("Decompress", func(t *testing.T) {
			got := Decompress(tC.in)
			assert.Equal(t, tC.want, got)
		})
	}
}
