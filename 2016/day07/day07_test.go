package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestSupportsTLS(t *testing.T) {
	testCases := []struct {
		in   string
		want bool
	}{
		{in: "abba[mnop]qrst", want: true},
		{in: "abcd[bddb]xyyx", want: false},
		{in: "aaaa[qwer]tyui", want: false},
		{in: "ioxxoj[asdfgh]zxcvbn", want: true},
	}
	for _, tC := range testCases {
		t.Run("solve1", func(t *testing.T) {
			got := SupportsTLS(tC.in)
			assert.Equal(t, tC.want, got)
		})
	}
}

func TestSupportsSSL(t *testing.T) {
	testCases := []struct {
		in   string
		want bool
	}{
		{in: "aba[bab]xyz", want: true},
		{in: "xyx[xyx]xyx", want: false},
		{in: "aaa[kek]eke", want: true},
		{in: "zazbz[bzb]cdb", want: true},
	}
	for _, tC := range testCases {
		t.Run("solve1", func(t *testing.T) {
			got := SupportsSSL(tC.in)
			assert.Equal(t, tC.want, got)
		})
	}
}
