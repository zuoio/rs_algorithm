use std::{cmp::Reverse, collections::BinaryHeap};

#[allow(dead_code)]
struct KthLargest {
    heap: BinaryHeap<Reverse<i32>>,
    top_k: usize,
}

#[allow(dead_code)]
impl KthLargest {
    pub fn new(k: i32, nums: Vec<i32>) -> Self {
        let top_k = k as usize;
        let mut heap = BinaryHeap::new();
        for num in nums {
            heap.push(Reverse(num));
            if heap.len() > top_k {
                heap.pop();
            }
        }
        KthLargest { heap, top_k }
    }

    pub fn add(&mut self, val: i32) -> i32 {
        self.heap.push(Reverse(val));
        if self.heap.len() > self.top_k {
            self.heap.pop();
        }
        self.heap.peek().unwrap().0
    }
}
