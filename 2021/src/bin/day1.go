package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
)

func loadDepths(fileName string) ([]int, error) {
	r, err := os.Open(fileName)
	if err != nil {
		return nil, err
	}

	defer r.Close()

	var ds []int
	s := bufio.NewScanner(r)
	s.Split(bufio.ScanWords)
	for s.Scan() {
		d, err := strconv.Atoi(s.Text())
		if err != nil {
			return ds, err
		}

		ds = append(ds, d)
	}

	return ds, s.Err()
}

func solvePart1(depths []int) uint {
	var count uint
	for i := 1; i < len(depths); i++ {
		if depths[i] > depths[i-1] {
			count++
		}
	}

	return count
}

func solvePart2(depths []int) uint {
	var ds []int
	for i := 2; i < len(depths); i++ {
		ds = append(ds, depths[i-2]+depths[i-1]+depths[i])
	}

	return solvePart1(ds)
}

func main() {
	depths, err := loadDepths("tests/day1/input")
	if err != nil {
		fmt.Fprintln(os.Stderr, err)
		os.Exit(1)
	}

	println(solvePart1(depths))
	println(solvePart2(depths))
}
