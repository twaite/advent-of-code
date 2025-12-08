#include "bank.h"
#include <istream>
#include <iostream>

int total_output_joltage(std::istream &input)
{
    std::string bank;
    int total = 0;

    while (getline(input, bank))
    {
        short a = 0;
        short b = 0;
        short i = 0;

        for (char battery : bank)
        {
            short d = battery - '0';
            ++i; // TODO: will this break the check below?

            if (d > a && i < bank.length())
            {
                a = d;
                b = 0;
                continue;
            }

            if (a > 0 && d > b)
            {
                b = d;
            }
        }

        short joltage = (a * 10) + b;
        total += joltage;

        std::cout << "joltage: " << joltage
                  << ", total: " << total
                  << std::endl;
    }

    return total;
}