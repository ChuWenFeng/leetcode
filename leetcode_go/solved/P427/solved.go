package P427

type Node struct {
	Val         bool
	IsLeaf      bool
	TopLeft     *Node
	TopRight    *Node
	BottomLeft  *Node
	BottomRight *Node
}

/*
 * @lc app=leetcode.cn id=427 lang=golang
 *
 * [427] 建立四叉树
 */

// @lc code=start
/**
 * Definition for a QuadTree node.
 * type Node struct {
 *     Val bool
 *     IsLeaf bool
 *     TopLeft *Node
 *     TopRight *Node
 *     BottomLeft *Node
 *     BottomRight *Node
 * }
 */

func construct(grid [][]int) *Node {
	len := len(grid)
	node := recursion(grid, 0, 0, len)
	return node
}

func recursion(grid [][]int, x, y, len int) *Node {
	if len == 1 {
		return &Node{
			Val:    grid[x][y] > 0,
			IsLeaf: true,
		}
	}

	nextlen := len / 2
	node := &Node{
		Val:         true,
		IsLeaf:      false,
		TopLeft:     recursion(grid, x, y, nextlen),
		TopRight:    recursion(grid, x, y+nextlen, nextlen),
		BottomLeft:  recursion(grid, x+nextlen, y, nextlen),
		BottomRight: recursion(grid, x+nextlen, y+nextlen, nextlen),
	}
	checkNode(node)
	return node
}

func checkNode(node *Node) {
	if !node.IsLeaf && node.TopLeft.IsLeaf && node.TopRight.IsLeaf && node.BottomLeft.IsLeaf && node.BottomRight.IsLeaf {
		if node.TopLeft.Val && node.TopRight.Val && node.BottomLeft.Val && node.BottomRight.Val {
			node.Val = node.TopLeft.Val
			node.IsLeaf = true
			node.TopLeft = nil
			node.TopRight = nil
			node.BottomLeft = nil
			node.BottomRight = nil
		} else if !node.TopLeft.Val && !node.TopRight.Val && !node.BottomLeft.Val && !node.BottomRight.Val {
			node.Val = node.TopLeft.Val
			node.IsLeaf = true
			node.TopLeft = nil
			node.TopRight = nil
			node.BottomLeft = nil
			node.BottomRight = nil
		}
	}
}

// @lc code=end
func Test(grid [][]int) *Node {
	return construct(grid)
}
