package main

import (
	"strings"
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestExecute(t *testing.T) {
	testCases := []struct {
		want []string
		in   []string
		w, h int
	}{
		{
			w: 7,
			h: 3,
			in: []string{
				"rect 1x1",
			},
			want: []string{
				"#......",
				".......",
				".......",
			},
		},
		{
			w: 7,
			h: 3,
			in: []string{
				"rect 1x1",
				"rotate row y=0 by 15",
			},
			want: []string{
				".#.....",
				".......",
				".......",
			},
		},
		{
			w: 7,
			h: 3,
			in: []string{
				"rect 1x1",
				"rotate column x=0 by 7",
			},
			want: []string{
				".......",
				"#......",
				".......",
			},
		},
		{
			w: 7,
			h: 3,
			in: []string{
				"rect 3x2",
			},
			want: []string{
				"###....",
				"###....",
				".......",
			},
		},
		{
			w: 7,
			h: 3,
			in: []string{
				"rect 3x2",
				"rotate column x=1 by 1",
			},
			want: []string{
				"#.#....",
				"###....",
				".#.....",
			},
		},
		{
			w: 7,
			h: 3,
			in: []string{
				"rect 3x2",
				"rotate column x=1 by 1",
				"rotate row y=0 by 4",
			},
			want: []string{
				"....#.#",
				"###....",
				".#.....",
			},
		},
		{
			w: 7,
			h: 3,
			in: []string{
				"rect 3x2",
				"rotate column x=1 by 1",
				"rotate row y=0 by 4",
				"rotate column x=1 by 1",
			},
			want: []string{
				".#..#.#",
				"#.#....",
				".#.....",
			},
		},
	}
	for _, tC := range testCases {
		t.Run("Execute", func(t *testing.T) {
			s := NewScreen(tC.w, tC.h)
			for _, l := range tC.in {
				s.Execute(l)
			}
			got := strings.TrimSpace(s.String())
			want := strings.Join(tC.want, "\n")
			assert.Equal(t, want, got)
		})
	}
}
