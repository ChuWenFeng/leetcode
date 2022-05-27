package p284

type Iterator struct {
}

func (this *Iterator) hasNext() bool {
	return false
}

func (this *Iterator) next() int {
	return 0
}

/*
 * @lc app=leetcode.cn id=284 lang=golang
 *
 * [284] 顶端迭代器
 */

// @lc code=start
/*   Below is the interface for Iterator, which is already defined for you.
 *
 *   type Iterator struct {
 *
 *   }
 *
 *   func (this *Iterator) hasNext() bool {
 *		// Returns true if the iteration has more elements.
 *   }
 *
 *   func (this *Iterator) next() int {
 *		// Returns the next element in the iteration.
 *   }
 */

type PeekingIterator struct {
	iter     *Iterator
	hashnext bool
	nex      int
}

func Constructor(iter *Iterator) *PeekingIterator {
	return &PeekingIterator{
		iter:     iter,
		hashnext: iter.hasNext(),
		nex:      iter.next(),
	}
}

func (this *PeekingIterator) hasNext() bool {
	return this.hashnext
}

func (this *PeekingIterator) next() int {
	ret := this.nex
	this.hashnext = this.iter.hasNext()
	if this.hashnext {
		this.nex = this.iter.next()
	}
	return ret
}

func (this *PeekingIterator) peek() int {
	return this.nex
}

// @lc code=end
