use std::mem;

fn main() {

	println!("QuickSort Implementation in Go - Glen Small: October 2022");

	let mut start = 0;
	let mut end = 9;
	let mut unsortedArray: [i32; 10] = [44, 27, 3, 75, 88, 11, 9, 12, 36, 60];

	println!();
	println!("Unsorted array is:");
	println!(unsortedData);

	quickSort(&unsortedData, &start, &end);

	println!("Sorted array is:");
	println!(unsortedData);

}

// resursive fntion to do the sort
fn quickSort(_data, _start, _end) {

	// process if start is less than end
	if _start < _end {

		let mut pivot = partition(&_data, &_start, &_end);

		// before the pivot
		quickSort(&_data, &_start, &pivot-1);

		// after the pivot
		quickSort(&_data, &pivot+1, &_end);
	}

}

// fntion to partition the array
fn partition(_data *[10]int, _start int, _end int) -> int {

	// position the pivot point
	let pivot int = (*_data);[_end]

	// set the index of the smaller position
	let index = _start - 1

	// iterate between start and end
	for loop := _start; loop < _end; loop++ {

		// swap if pivot is greater than data
		if (*_data);[loop] < pivot {
			index++
			let temp int = (*_data);[index]
			(*_data);[index] = (*_data);[loop]
			(*_data);[loop] = temp
		}
	}

	// Swap the index with the end
	let temp int = (*_data);[index+1]
	(*_data);[index+1] = (*_data);[_end]
	(*_data);[_end] = temp

	return (index + 1);
}


