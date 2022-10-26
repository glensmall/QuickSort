def partition(_data, _start, _end):

    # position the pivot point
	pivot = _data[_end]

	# set the index of the smaller position
	index = _start - 1

	# iterate between start and end
	for loop in range(_start, _end) :

		# swap if pivot is greater than data
		if _data[loop] < pivot:
			index = index+1
			temp = _data[index]
			_data[index] = _data[loop]
			_data[loop] = temp
		
	

	# Swap the index with the end
	temp = _data[index+1]
	_data[index+1] = _data[_end]
	_data[_end] = temp

	return (index + 1)



def quickSort(_data, _start, _end):

    if _start < _end :

        pivot = partition(_data, _start, _end)

        # before the pivot
        quickSort(_data, _start, pivot-1)

        # after the pivot
        quickSort(_data, pivot+1, _end)
	



if __name__ == "__main__":

    print("QuickSort Implementation in Python - Glen Small: October 2022")

    start = 0
    end = 9
    unsortedData = [44, 27, 3, 75, 88, 11, 9, 12, 36, 60]

    print(f"Unsorted array is: {unsortedData}")

    quickSort(unsortedData, start, end)

    print(f"Sorted array is: {unsortedData}")

