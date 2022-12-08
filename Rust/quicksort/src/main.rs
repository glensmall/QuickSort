
fn main() 
{

	println!("quick_sort Implementation in Rust - Glen Small: October 2022");

	let mut start: usize = 0;
	let mut end: usize = 9;
	let mut unsorted_array: [i32; 10] = [44, 27, 3, 75, 88, 11, 9, 12, 36, 60];

	println!();
	println!("UnSorted array is: {:?}", unsorted_array);

	quick_sort(&mut unsorted_array, &mut start, &mut end);

	println!("Sorted array is: {:?}", unsorted_array);

}

// resursive fntion to do the sort
fn quick_sort(_data: &mut [i32; 10], _start: &mut usize, _end: &mut usize) 
{

	// process if start is less than end
	if *_start < *_end 
	{

		let pivot: usize = partition(_data, _start, _end);

		// before the pivot
		quick_sort(_data, _start, &mut (pivot -1));

		// after the pivot
		quick_sort(_data, &mut (pivot +1), _end);
	}

}

// fntion to partition the array
fn partition(_data: &mut [i32; 10], _start: &mut usize, _end: &mut usize) -> usize
{
	// position the pivot point
	let pivot: i32 = _data[*_end];

	// set the index of the smaller position
	let mut index = *_start - 1;

	// iterate between start and end
	for _loop in *_start..*_end
	{
		// swap if pivot is greater than data
		if _data[_loop] < pivot 
		{
			index = index+1;
			let mut _temp = _data[index];
			_data[index] = _data[_loop];
			_data[_loop] = _temp;
		}
	}

	// Swap the index with the end
	let temp: i32 = _data[index+1];
	_data[index+1] = _data[*_end];
	_data[*_end] = temp;

	// return value
	index + 1
}


