#![allow(dead_code)]

pub fn quick_sort<T>(array: &mut Vec<T>, start: usize, end: usize)
where
	T: std::cmp::PartialOrd,
{
	if start+1 >= end {
		return;
	}

	let mut j: usize = start;
	for i in start..end-1 {
		if array[i] <= array[end-1] {
			array.swap(i, j);
			j+=1;
		}
	}
	array.swap(end-1, j);

	quick_sort(array, start, j);
	quick_sort(array, j+1, end);
}

#[cfg(test)]
mod quick_sort_tests {
	use super::quick_sort;
	
}

fn bubble_sort<T>(array: &mut Vec<T>)
where
	T: std::cmp::PartialOrd + std::fmt::Debug,
{
	if array.len() < 2 {
		return;
	}
	
	let mut run: bool = true;
	while run {
		run = false;
		for i in 0..array.len()-1 {
			if array[i] > array[i+1] {
				array.swap(i, i+1);
				run = true;
			}
		}
	}
}

fn merge_sort<T>(array: &mut Vec<T>, start: usize, end: usize)
where
	T: std::cmp::PartialOrd + std::fmt::Debug,
{
	let mut middle: usize = (end + start) / 2;
	if end - start >= 3 {
		merge_sort(array, start, middle);
		merge_sort(array, middle, end);
	}

	let mut i: usize = start;
	let mut j: usize = middle;
	while i < middle && j < end {
		if array[i] > array[j] {
			let data: T = array.remove(j);
			array.insert(i, data);
			i+=1;
			j+=1;
			middle+=1;
		}
		else {
			i+=1;
		}
	}
}
