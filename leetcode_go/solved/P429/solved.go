package p429

type Node struct {
	Val      int
	Children []*Node
}

/*
 * @lc app=leetcode.cn id=429 lang=golang
 *
 * [429] N 叉树的层序遍历
 */

// @lc code=start
/**
 * Definition for a Node.
 * type Node struct {
 *     Val int
 *     Children []*Node
 * }
 */

func levelOrder(root *Node) [][]int {
	var ans [][]int
	if root == nil {
		return ans
	}
	var queue []*Node
	queue = append(queue, root)
	idx := 0
	layercount := 1
	for layercount > 0 {
		layer := make([]int, 0)
		count := 0
		for layercount > 0 {
			node := queue[idx]
			layer = append(layer, node.Val)
			queue = append(queue, node.Children...)
			count += len(node.Children)
			layercount -= 1
			idx += 1
		}
		layercount = count
		ans = append(ans, layer)
	}

	return ans
}

// @lc code=end
