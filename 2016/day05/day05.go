package main

import (
	"crypto/md5"
	"encoding/hex"
	"fmt"
)

func makeHash(key string, index int) string {
	s := []byte(fmt.Sprintf("%s%d", key, index))
	hash := md5.Sum(s)
	return hex.EncodeToString(hash[:])
}

func startsWith5Zeroes(hash string) bool {
	return hash[0] == '0' &&
		hash[1] == '0' &&
		hash[2] == '0' &&
		hash[3] == '0' &&
		hash[4] == '0'
}

func solve1(line string) string {
	var password []byte
	for index := 0; len(password) < 8; index++ {
		h := makeHash(line, index)
		if startsWith5Zeroes(h) {
			password = append(password, h[5])
		}
	}
	return string(password)
}

func solve2(line string) string {
	password := []byte{'x', 'x', 'x', 'x', 'x', 'x', 'x', 'x'}
	found := 0
	for index := 0; found < 8; index++ {
		h := makeHash(line, index)
		if startsWith5Zeroes(h) {
			pos := int(h[5]) - '0'
			if pos < 0 || pos > 7 {
				continue
			}
			if password[pos] != 'x' {
				continue
			}
			password[pos] = h[6]
			found++
		}
	}
	return string(password)
}

func main() {
	line := "reyedfim"
	fmt.Println(solve1(line))
	fmt.Println(solve2(line))
}
