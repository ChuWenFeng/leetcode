package iv0406

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

/**
 * Definition for a binary tree node.
 * type TreeNode struct {
 *     Val int
 *     Left *TreeNode
 *     Right *TreeNode
 * }
 */
func inorderSuccessor(root *TreeNode, p *TreeNode) *TreeNode {
	var success *TreeNode
	if p.Right != nil {
		success = p.Right
		for success.Left != nil {
			success = success.Left
		}
		return success
	}

	node := root
	for node != nil {
		if node.Val > p.Val {
			success = node
			node = node.Left
		} else {
			node = node.Right
		}
	}

	return success
}
