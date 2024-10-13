package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestValidateIsRealRoom(t *testing.T) {
	testCases := []struct {
		s         string
		want_id   int
		want_real bool
	}{
		{
			s:         "aaaaa-bbb-z-y-x-123[abxyz]",
			want_id:   123,
			want_real: true,
		},
		{
			s:         "a-b-c-d-e-f-g-h-987[abcde]",
			want_id:   987,
			want_real: true,
		},
		{
			s:         "not-a-real-room-404[oarel]",
			want_id:   404,
			want_real: true,
		},
		{
			s:         "totally-real-room-200[decoy]",
			want_id:   200,
			want_real: false,
		},
	}
	for _, tC := range testCases {
		t.Run("IsRealRoom", func(t *testing.T) {
			id, real := IsRealRoom(tC.s)
			assert.Equal(t, tC.want_id, id)
			assert.Equal(t, tC.want_real, real)
		})
	}
}

func TestValidateShiftLetter(t *testing.T) {
	testCases := []struct {
		in, want byte
	}{
		{in: 'a', want: 'b'},
		{in: 'c', want: 'd'},
		{in: 'o', want: 'p'},
		{in: 'y', want: 'z'},
		{in: 'z', want: 'a'},
		{in: '-', want: ' '},
	}
	for _, tC := range testCases {
		t.Run("ShiftLetter", func(t *testing.T) {
			got := shiftLetter(tC.in, 1)
			assert.Equalf(t, tC.want, got, "shiftLetter('%c', 1) == '%c'", tC.in, got)
		})
	}
}

func TestValidateDecryptRoomName(t *testing.T) {
	testCases := []struct {
		in, want string
	}{
		{
			in:   "qzmt-zixmtkozy-ivhz-343[abcd]",
			want: "very encrypted name",
		},
	}
	for _, tC := range testCases {
		t.Run("DecryptRoomName", func(t *testing.T) {
			got := DecryptRoomName(tC.in)
			assert.Equal(t, tC.want, got)
		})
	}
}

func TestValidateSolve1(t *testing.T) {
	testCases := []struct {
		in   []string
		want int
	}{
		{
			in: []string{
				"aaaaa-bbb-z-y-x-123[abxyz]",
				"a-b-c-d-e-f-g-h-987[abcde]",
				"not-a-real-room-404[oarel]",
				"totally-real-room-200[decoy]",
			},
			want: 1514,
		},
	}
	for _, tC := range testCases {
		t.Run("solve1", func(t *testing.T) {
			got := solve1(tC.in)
			assert.Equal(t, tC.want, got)
		})
	}
}
