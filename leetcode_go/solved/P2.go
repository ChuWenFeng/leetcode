package solved

type ListNode struct {
	Val  int
	Next *ListNode
}

/*
 * @lc app=leetcode.cn id=2 lang=golang
 *
 * [2] 两数相加
 */

// @lc code=start
/**
 * Definition for singly-linked list.

 */
func addTwoNumbers(l1 *ListNode, l2 *ListNode) *ListNode {
	carry := 0
	head := l1

	v := l1.Val + l2.Val
	carry = v / 10
	v = v % 10
	l1.Val = v

	for l1.Next != nil && l2.Next != nil {
		l1 = l1.Next
		l2 = l2.Next

		val := l1.Val + l2.Val + carry
		carry = val / 10
		val = val % 10
		l1.Val = val
	}

	for l1.Next != nil {
		l1 = l1.Next
		val := l1.Val + carry
		carry = val / 10
		val = val % 10
		l1.Val = val
	}

	for l2.Next != nil {
		l2 = l2.Next

		val := l2.Val + carry
		carry = val / 10
		val = val % 10
		// l1.Val = val
		l1.Next = &ListNode{Val: val, Next: nil}
		l1 = l1.Next
	}

	if carry != 0 {
		l1.Next = &ListNode{Val: carry, Next: nil}
	}

	return head
}

// @lc code=end
