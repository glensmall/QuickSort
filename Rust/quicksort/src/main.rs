
fn main() 
{

	println!("quick_sort Implementation in Rust - Glen Small: October 2022");

	let start: usize = 0;
	let  end: usize = 9;
	let mut unsorted_array: [i32; 10] = [44, 27, 3, 75, 88, 11, 9, 12, 36, 60];

	println!();
	println!("UnSorted array is: {:?}", unsorted_array);

	quick_sort(&mut unsorted_array, start, end);

	println!("Sorted array is: {:?}", unsorted_array);

}

// resursive fntion to do the sort
fn quick_sort(_data: &mut [i32; 10], _start: usize, _end: usize) 
{
	println!("QUICKSORT - Start = {}  End = {}", _start, _end);

	// process if start is less than end
	if _start < _end 
	{
		println!("Pivoting with {} and {}", _start, _end);
		let pivot: usize = partition(_data, _start, _end);
		println!("Partition returned {}", pivot);

		// before the pivot
		let before: usize = pivot - 1;
		println!("BEFORE {}", before);
		quick_sort(_data, _start,  before);

		// after the pivot

		let after: usize = pivot + 1;
		println!("AFTER {}", after);
		quick_sort(_data,  after, _end);
	}

}

// fntion to partition the array
fn partition(_data: &mut [i32; 10], _start: usize, _end: usize) -> usize
{
	println!("PARTITION - Start = {}  End = {}", _start, _end);

	// position the pivot point
	let pivot: i32 = _data[_end];
	println!("pivot = {}", pivot);

	// set the index of the smaller position
	let mut index = _start;
	println!("index = {}", index);

	let mut _loop = _start;
	// iterate between start and end
	while _loop <= _end
	{
		println!("loop = {} loop array index data = {} pivot = {}", _loop, _data[_loop], pivot);
		// swap if pivot is greater than data
		if _data[_loop] < pivot 
		{
			println!("Swapping {} with {}", _data[_loop], pivot);
			let mut _temp = *_data[index];
			*_data[index] = *_data[_loop];
			*_data[_loop] = _temp;
			index = index+1;
		}

		_loop = _loop + 1;
	}

	// Swap the index with the end
	let temp: i32 = _data[index+1];
	_data[index+1] = _data[_end];
	_data[_end] = temp;

	println!("array after partition is: {:?}", _data);

	// return value
	index + 1
}


