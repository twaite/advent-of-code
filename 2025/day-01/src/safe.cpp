#include "safe.h"
#include <string>
#include <iostream>

int turnDial(const std::istream &input)
{
    std::istream &in = const_cast<std::istream &>(input);
    int count = 0;
    int dial = 50;
    std::string line;

    while (std::getline(in, line))
    {
        char direction = line[0];
        int steps = stoi(line.substr(1, line.length()));

        if (direction == 'L')
        {
            dial = (dial - steps) % 100;
        }
        else if (direction == 'R')
        {
            dial = (dial + steps) % 100;
        }
        else
        {
            std::cerr << "Invalid direction: " << direction << std::endl;
        }

        if (dial == 0)
        {
            count++;
        }

        std::cout << "Direction: " << direction << ", Steps: " << steps << ", Dial: " << dial << ", Count: " << count << std::endl;
    }

    return count;
}
