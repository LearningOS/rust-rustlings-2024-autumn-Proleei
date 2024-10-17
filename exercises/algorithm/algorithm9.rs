/*
	heap
	This question requires you to implement a binary heap function
*/

use std::cmp::Ord;
use std::default::Default;

pub struct Heap<T>
where
    T: Default,
{
    count: usize,
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

impl<T> Heap<T>
where
    T: Default
{
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            count: 0,
            items: vec![T::default()],
            comparator,
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn add(&mut self, value: T) {
        //TODO
        // self.count += 1;
        // let mut child = self.len();
        // let mut parent = self.parent_idx(child.clone());
        // self.items.push(value);
        // while (self.comparator)(&self.items[child], &self.items[parent]) {
        //     //let tmp = self.items[child];
        //     std::mem::swap(&mut self.items[child], &mut self.items[parent]);
        //     child = parent;
        //     parent = self.parent_idx(child);
        // }
                // 增加元素到向量末尾
                self.items.push(value);
                self.count += 1;
        
                // 上浮新元素以维持堆的性质
                let mut idx = self.len();
                while idx > 1 && (self.comparator)(&self.items[idx], &self.items[self.parent_idx(idx)]) {
                    let parent_idx = self.parent_idx(idx);
                    self.items.swap(idx, parent_idx);
                    idx = self.parent_idx(idx);
                }
    }

    fn parent_idx(&self, idx: usize) -> usize {
        idx / 2
    }

    fn children_present(&self, idx: usize) -> bool {
        self.left_child_idx(idx) <= self.count
    }

    fn left_child_idx(&self, idx: usize) -> usize {
        idx * 2
    }

    fn right_child_idx(&self, idx: usize) -> usize {
        self.left_child_idx(idx) + 1
    }

    fn smallest_child_idx(&self, idx: usize) -> usize {
        //TODO
        let left = self.left_child_idx(idx);
        let right = self.right_child_idx(idx);

        // 只有左子节点存在的情况
        if right > self.count {
            return left;
        }

        // 比较左右子节点，返回较小的那个
        if (self.comparator)(&self.items[left], &self.items[right]) {
            left
        } else {
            right
        }
    }

    fn bubble_down(&mut self, mut idx: usize) {
        while self.children_present(idx) {
            let smallest_child_idx = self.smallest_child_idx(idx);

            if !(self.comparator)(&self.items[smallest_child_idx], &self.items[idx]) {
                break;
            }

            self.items.swap(idx, smallest_child_idx);
            idx = smallest_child_idx;
        }
    }

}

impl<T> Heap<T>
where
    T: Default + Ord,
{
    /// Create a new MinHeap
    pub fn new_min() -> Self {
        Self::new(|a, b| a < b)
    }

    /// Create a new MaxHeap
    pub fn new_max() -> Self {
        Self::new(|a, b| a > b)
    }
}

impl<T> Iterator for Heap<T>
where
    T: Default+Clone,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        //TODO
		if self.is_empty() {
            return None;
        }

        // 获取堆顶元素（最小值或最大值）
        let top = self.items[1].clone();

        // 将最后一个元素移到堆顶，并减少堆大小
        let last = self.items.pop().unwrap();
        self.count -= 1;

        // 如果堆不为空，将最后一个元素放到堆顶并下沉
        if !self.is_empty() {
            self.items[1] = last;
            self.bubble_down(1);
        }

        Some(top)
    }
}

pub struct MinHeap;

impl MinHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a < b)
    }
}

pub struct MaxHeap;

impl MaxHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a > b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_empty_heap() {
        let mut heap = MaxHeap::new::<i32>();
        assert_eq!(heap.next(), None);
    }

    #[test]
    fn test_min_heap() {
        let mut heap = MinHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(2));
        assert_eq!(heap.next(), Some(4));
        assert_eq!(heap.next(), Some(9));
        heap.add(1);
        assert_eq!(heap.next(), Some(1));
    }

    #[test]
    fn test_max_heap() {
        let mut heap = MaxHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(11));
        assert_eq!(heap.next(), Some(9));
        assert_eq!(heap.next(), Some(4));
        heap.add(1);
        assert_eq!(heap.next(), Some(2));
    }
}