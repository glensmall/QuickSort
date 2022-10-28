using System;

namespace Quicksort
{
    class Program
    {
        static void Main(string[] args)
        {
            // a few variables to use
            int Start = 0;
            int End = 9;
            int []unsortedArray = { 44, 27, 3, 75, 88, 11, 9, 12, 36, 60 };

            // let the user know whats going on
            Console.Write("The Unsorted data is: {");

            foreach(int index in unsortedArray)
            {
                Console.Write($"{index}, ");
            }

            Console.WriteLine("}");

            // now sort the array
            quickSort(ref unsortedArray, Start, End);

            // let the user know whats going on
            Console.Write("\nThe Sorted data is: {");

            foreach (int index in unsortedArray)
            {
                Console.Write($"{index}, ");
            }

            Console.WriteLine("}");

            return;
        }

        // Recursive method to the the sorting
        static void quickSort(ref int[]_data, int _start, int _end)
        {
            // process if start is less than end
            if (_start < _end)
            {
                int pivot = partition(ref _data, _start, _end);

                // before the pivot
                quickSort(ref _data, _start, pivot - 1);

                // after the pivot
                quickSort(ref _data, pivot + 1, _end);
            }
        }

        // Method to Partition the array
        static int partition(ref int []_data, int _start, int _end)
        {

            // position the pivot point
            int pivot = _data[_end];

            // set the index of the smaller position
            int index = _start - 1;

            // iterate between start and end
            for (int loop = _start; loop < _end; loop++)
            {

                // swap if pivot is greater than data
                if (_data[loop] < pivot)
                {
                    index++;
                    int tmp = _data[index];
                    _data[index] = _data[loop];
                    _data[loop] = tmp;
                }
            }

            // Swap the index with the end
            int temp = _data[index + 1];
            _data[index + 1] = _data[_end];
            _data[_end] = temp;

            return (index + 1);
        }
    }
}
