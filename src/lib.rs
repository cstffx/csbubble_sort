#![feature(is_sorted)]

use std::fmt::Display;

trait BubbleSort<T> {
    fn bubble_sort(&mut self);
    fn bubble_sort_by(&mut self, by: FnMustSwap<T>);
}

type FnMustSwap<T> = fn(a: &T, b: &T) -> bool;

impl<T> BubbleSort<T> for Vec<T> where T: PartialOrd {
    fn bubble_sort(&mut self) {
        self.bubble_sort_by(|a, b| a > b)
    }

    fn bubble_sort_by(&mut self, by: FnMustSwap<T>) {
        let mut stop = true;
        for i in 0..self.len() {
            for j in 0..(self.len() - i - 1) {
                if by(&self[j], &self[j + 1]) {
                    self.swap(j, j + 1);
                    stop = false;
                }
            }
            if stop {
                break;
            }
            stop = true;
        }
    }
}

#[cfg(test)]
mod test {
    use crate::BubbleSort;

    #[test]
    fn test_bubble_sort() {
        let mut input: Vec<u32> = vec![];
        input.reverse();
        input.bubble_sort();
        assert!(input.is_sorted());

        let mut input: Vec<u32> = vec![1];
        input.reverse();
        input.bubble_sort();
        assert!(input.is_sorted());

        let mut input: Vec<u32> = vec![2, 1];
        input.reverse();
        input.bubble_sort();
        assert!(input.is_sorted());

        let mut input: Vec<u32> = vec![0, 1, 2, 3];
        input.reverse();
        input.bubble_sort();
        assert!(input.is_sorted());
    }
}