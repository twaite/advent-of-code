#include "safe.h"
#include <string>
#include <iostream>

int normalize(int val)
{
    int n = val % 100;
    if (n < 0)
        n += 100;
    return n;
}

int turnDial(const std::istream &input, bool onlyIfTurnEndsOnZero)
{
    std::istream &in = const_cast<std::istream &>(input);
    int count = 0;
    int dial = 50;
    std::string line;

    while (std::getline(in, line))
    {
        char direction = line[0];
        int steps = stoi(line.substr(1));
        int wraps = 0;

        if (direction == 'L')
        {
            wraps = ((100 - dial) % 100 + steps) / 100;
            dial = normalize(dial - steps);
        }
        else if (direction == 'R')
        {
            wraps = (dial + steps) / 100;
            dial = normalize(dial + steps);
        }
        else
        {
            std::cerr << "Invalid direction: " << direction << std::endl;
        }

        if (onlyIfTurnEndsOnZero)
        {
            if (dial == 0)
            {
                count++;
            }
        }
        else
        {
            count += wraps;
        }

        std::cout << "Direction: " << direction
                  << ", Steps: " << steps
                  << ", Dial: " << dial
                  << ", Wraps: " << wraps
                  << ", Count: " << count << std::endl;
    }

    return count;
}
