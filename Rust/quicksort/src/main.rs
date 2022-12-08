
fn main() 
{

	println!("QuickSort Implementation in Go - Glen Small: October 2022");

	let mut start = 0;
	let mut end = 9;
	let mut unsortedArray: [i32; 10] = [44, 27, 3, 75, 88, 11, 9, 12, 36, 60];

	println!();
	println!("Unsorted array is:");
	//println!(unsortedData);

	quickSort(&mut unsortedArray, &mut start, &mut end);

	println!("Sorted array is:");
	//println!(unsortedData);

}

// resursive fntion to do the sort
fn quickSort(_data: &mut [i32; 10], _start: &mut i32, _end: &mut i32) 
{

	// process if start is less than end
	if *_start < *_end 
	{

		let mut pivot = partition(_data, _start, _end);

		// before the pivot
		pivot-1;
		quickSort(_data, _start, pivot);

		// after the pivot
		pivot+1;
		quickSort(_data, pivot, _end);
	}

}

// fntion to partition the array
fn partition(_data: &mut [i32; 10], _start: &mut i32, _end: &mut i32) -> i32
{

	// position the pivot point
	let mut pivot = _data[_end];

	// set the index of the smaller position
	let mut index = *_start - 1;

	// iterate between start and end
	for _loop in _start.._end
	{
		// swap if pivot is greater than data
		if _data[_loop] < pivot 
		{
			index = index+1;
			let mut _temp = _data[index];
			_data[index] = _data[_loop];
			_data[_loop] = temp;
		}
	}

	// Swap the index with the end
	let mut temp int = _data[index+1];
	_data[index+1] = _data[_end];
	_data[_end] = temp;

	// return value
	index + 1
}


