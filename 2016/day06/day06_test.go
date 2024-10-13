package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestValidateSolve1(t *testing.T) {
	testCases := []struct {
		want string
		in   []string
	}{
		{
			in: []string{
				"eedadn",
				"drvtee",
				"eandsr",
				"raavrd",
				"atevrs",
				"tsrnev",
				"sdttsa",
				"rasrtv",
				"nssdts",
				"ntnada",
				"svetve",
				"tesnvt",
				"vntsnd",
				"vrdear",
				"dvrsen",
				"enarar",
			},
			want: "easter",
		},
	}
	for _, tC := range testCases {
		t.Run("solve1", func(t *testing.T) {
			got := solve1(tC.in)
			assert.Equal(t, tC.want, got)
		})
	}
}
