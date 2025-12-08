#include <iostream>
#include <string>
#include <fstream>
#include "bank.h"

int main()
{
    std::ifstream input("input.txt");
    if (!input.is_open())
    {
        std::cerr << "Error: Could not open input file" << std::endl;
        return 1;
    }

    int count = total_output_joltage(input);

    std::cout << "Count is: " << count
              << std::endl;
}