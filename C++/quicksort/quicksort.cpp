#include "quicksort.h"

int main()
{
    std::cout << "QuickSort Implementation in C++ - Glen Small : October 2022" << std::endl;

    // a few variables to use
    int Start = 0;
    int End = 9;
    std::array<int, 10> unsortedArray = { 44, 27, 3, 75, 88, 11, 9, 12, 36, 60 };

    // let the user know whats going on
    std::cout << std::endl << "The Unsorted data is: {";

    for (auto index : unsortedArray)
    {
        std::cout << index << ' ';
    }

    std::cout << "}" << std::endl;

    // now sort the array
    quickSort(&unsortedArray, Start, End);

    // let the user know whats going on
    std::cout << std::endl << "The Sorted data is: {";

    for (auto index : unsortedArray)
    {
        std::cout << index << ' ';
    }

    std::cout << "}" << std::endl;

    return(0);   
}

void quickSort(std::array<int, 10>* _data, int _start, int _end)
{
    // process if start is less than end
    if (_start < _end)
    {
        int pivot = partition(_data, _start, _end);

        // before the pivot
        quickSort(_data, _start, pivot - 1);

        // after the pivot
        quickSort(_data, pivot + 1, _end);
    }
}

int partition(std::array<int, 10>* _data, int _start, int _end)
{

    // position the pivot point
    int pivot = _data->data()[_end];

    // set the index of the smaller position
    int index = _start - 1;

    // iterate between start and end
    for(int loop = _start; loop < _end; loop++) 
    {

        // swap if pivot is greater than data
        if(_data->data()[loop] < pivot)
        {
            index++;
            int temp = _data->data()[index];
            _data->data()[index] = _data->data()[loop];
            _data->data()[loop] = temp;
        }
    }

    // Swap the index with the end
    int temp = _data->data()[index + 1];
    _data->data()[index + 1] = _data->data()[_end];
    _data->data()[_end] = temp;

    return (index + 1);
}