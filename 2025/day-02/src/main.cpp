#include <iostream>
#include <string>
#include <fstream>
#include "product.h"

int main()
{
    std::ifstream input("input.txt");
    if (!input.is_open())
    {
        std::cerr << "Error: Could not open input file" << std::endl;
        return 1;
    }

    long count = count_invalid_ids(input);

    std::cout << "Count is: " << count
              << std::endl;
}