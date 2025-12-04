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
    int count = turnDial(input);
    std::cout << "Count is: " << count << std::endl;
}