package main

import "fmt"

func main() {

	fmt.Println("QuickSort Implementation in Go - Glen Small: October 2022")

	var start int
	var end int
	unsortedData := [10]int{4, 7, 3, 5, 8, 1, 9, 2, 6, 0}

}

// resursive function to do the sort
func quickSortRecursive(_data *[10]int, _start int, _end int) {

}

// function to partition the array
func partitionArray(_data *[10]int, _start int, _end int) int {

	// position the pivot point
	var pivot int = _data[_end]

	// set the index of the smaller position
	var index = _start - 1

	// iterate between start and end
	for loop := _start; loop < _end; loop++ {

		// swap if pivot is greater than data
		if _data[loop] < pivot {
			index++
			var temp int = _data[index]
			_data[index] = _data[loop]
			_data[loop] = temp
		}
	}

	var temp int = _data[index+1]
	_data[index+1] = _data[_end]
	_data[_end] = temp

	return (index + 1)
}
