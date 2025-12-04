#include <iostream>
#include <string>
#include <fstream>
#include "safe.h"

int main()
{
    std::ifstream input("input.txt");
    if (!input.is_open())
    {
        std::cerr << "Error: Could not open input file" << std::endl;
        return 1;
    }

    int a = turnDial(input, true);

    // Reset to beginning of file stream
    input.clear();
    input.seekg(0, std::ios::beg);

    int b = turnDial(input, false);

    std::cout << "Count is: " << a
              << ", All zeros: " << b
              << std::endl;
}