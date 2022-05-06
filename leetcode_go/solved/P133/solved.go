package p133

type Node struct {
	Val       int
	Neighbors []*Node
}

/*
 * @lc app=leetcode.cn id=133 lang=golang
 *
 * [133] 克隆图
 */

// @lc code=start
/**
 * Definition for a Node.
 * type Node struct {
 *     Val int
 *     Neighbors []*Node
 * }
 */

func cloneGraph(node *Node) *Node {
	flag := make(map[*Node]*Node)
	return dfs(node, &flag)
}

func dfs(node *Node, flag *map[*Node]*Node) *Node {
	if node == nil {
		return node
	}
	if node, ok := (*flag)[node]; ok {
		return node
	}
	newnode := &Node{node.Val, []*Node{}}
	(*flag)[node] = newnode
	for _, n := range node.Neighbors {
		newnode.Neighbors = append(newnode.Neighbors, dfs(n, flag))
	}
	return newnode
}

// @lc code=end
