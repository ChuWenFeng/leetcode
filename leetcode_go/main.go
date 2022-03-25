package main

import (
	"fmt"
)

type listslice [1]int

type Point struct {
	val int
}

func main() {
	fmt.Println("largest number 10, 2 =", LargestNumber([]int{3, 30, 34, 5, 9}))

	m := make(map[Point]int)
	v1 := Point{val: 1}
	v2 := Point{val: 2}
	m[v1] = 1
	m[v2] = 2
	ml := make(map[listslice]int)
	// ml := make(listslice, 0)
	// ml = append(ml, 1)
	println(m[v1])
	// println(ml[0])

}

func LargestNumber(list []int) string {
	var ans = ""
	len := len(list)
	dp := make([][]int, len+1)
	for i := 0; i < len; i++ {
		dp[i] = make([]int, len+1)
	}

	for i := 1; i < len; i++ {

	}

	return ans
}
