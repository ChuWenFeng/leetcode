package solved

import "testing"

type Solution struct{}

func Test_addTwoNumbers(t *testing.T) {
	h1 := ListNode{Val: 2, Next: &ListNode{Val: 4, Next: &ListNode{Val: 9, Next: nil}}}
	h2 := ListNode{Val: 5, Next: &ListNode{Val: 6, Next: &ListNode{Val: 4, Next: &ListNode{Val: 9, Next: &ListNode{Val: 9, Next: nil}}}}}

	addTwoNumbers(&h1, &h2)

}
