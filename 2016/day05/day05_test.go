package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestValidateMakeHash(t *testing.T) {
	testCases := []struct {
		key, want string
		index     int
	}{
		{
			key: "abc", index: 3231929,
			want: "00000155f8105dff7f56ee10fa9b9abd",
		},
		{
			key: "abc", index: 5017308,
			want: "000008f82c5b3924a1ecbebf60344e00",
		},
		{
			key: "abc", index: 5278568,
			want: "00000f9a2c309875e05c5a5d09f1b8c4",
		},
	}
	for _, tC := range testCases {
		t.Run("makeHash", func(t *testing.T) {
			got := makeHash(tC.key, tC.index)
			assert.Equal(t, tC.want, got)
		})
	}
}

func TestValidateSolve1(t *testing.T) {
	testCases := []struct {
		in, want string
	}{
		{
			in:   "abc",
			want: "18f47a30",
		},
	}
	for _, tC := range testCases {
		t.Run("solve1", func(t *testing.T) {
			got := solve1(tC.in)
			assert.Equal(t, tC.want, got)
		})
	}
}

func TestValidateSolve2(t *testing.T) {
	testCases := []struct {
		in, want string
	}{
		{
			in:   "abc",
			want: "05ace8e3",
		},
	}
	for _, tC := range testCases {
		t.Run("solve2", func(t *testing.T) {
			got := solve2(tC.in)
			assert.Equal(t, tC.want, got)
		})
	}
}
