package solved

import (
	"leetcode_go/solved/P427"
	p591 "leetcode_go/solved/P591"
	"testing"
)

type Solution struct{}

func Test_addTwoNumbers(t *testing.T) {
	h1 := ListNode{Val: 2, Next: &ListNode{Val: 4, Next: &ListNode{Val: 9, Next: nil}}}
	h2 := ListNode{Val: 5, Next: &ListNode{Val: 6, Next: &ListNode{Val: 4, Next: &ListNode{Val: 9, Next: &ListNode{Val: 9, Next: nil}}}}}

	addTwoNumbers(&h1, &h2)

}

func Test_p427(t *testing.T) {
	grid := [][]int{
		{1, 1, 0, 0},
		{0, 0, 1, 1},
		{1, 1, 0, 0},
		{0, 0, 1, 1}}
	P427.Test(grid)
}

func Test_p591(t *testing.T) {
	code := "<A></A><B></B>"
	ans := false
	res := p591.Test(code)
	if res != ans {
		t.Fail()
	}
}
