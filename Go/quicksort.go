package main

import "fmt"

func main() {

	fmt.Println("QuickSort Implementation in Go - Glen Small: October 2022")

	var start int = 0
	var end int = 9
	unsortedData := [10]int{44, 27, 3, 75, 88, 11, 9, 12, 36, 60}

	fmt.Println()
	fmt.Println("Unsorted array is:")
	fmt.Println(unsortedData)

	quickSort(&unsortedData, start, end)

	fmt.Println("Sorted array is:")
	fmt.Println(unsortedData)

}

// resursive function to do the sort
func quickSort(_data *[10]int, _start int, _end int) {

	// process if start is less than end
	if _start < _end {

		var pivot = partition(_data, _start, _end)

		// before the pivot
		quickSort(_data, _start, pivot-1)

		// after the pivot
		quickSort(_data, pivot+1, _end)
	}

}

// function to partition the array
func partition(_data *[10]int, _start int, _end int) int {

	// position the pivot point
	var pivot int = (*_data)[_end]

	// set the index of the smaller position
	var index = _start

	// iterate between start and end
	for loop := _start; loop < _end; loop++ {

		// swap if pivot is greater than data
		if (*_data)[loop] < pivot {
			
			var temp int = (*_data)[index]
			(*_data)[index] = (*_data)[loop]
			(*_data)[loop] = temp
			index++
		}
	}

	// Swap the index with the end
	var temp int = (*_data)[index]
	(*_data)[index] = (*_data)[_end]
	(*_data)[_end] = temp

	return (index)
}
