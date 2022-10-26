#pragma once
#include <array>
#include <iostream>

void quickSort(std::array<int, 10>* _data, int _start, int _end);
int partition(std::array<int, 10>* _data, int _start, int _end);