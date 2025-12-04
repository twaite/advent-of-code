#include "product.h"
#include <regex>
#include <istream>
#include <iostream>

long count_invalid_ids(std::istream &input)
{
    char del = ',';
    std::regex re(R"((\d+)-(\d+))");

    long total = 0;
    std::string products;

    while (getline(input, products, del))
    {
        std::smatch match;

        if (std::regex_match(products, match, re))
        {
            long from = std::stoull(match[1].str());
            long to = std::stoull(match[2].str());

            for (long i = from; i <= to; i++)
            {
                int digits = log10(i) + 1;
                long divisor = pow(10, digits / 2);
                if (i / divisor == i % divisor)
                {
                    total += i;
                }

                // std::cout << "From: " << from
                //           << ", To: " << to
                //           << ", Count: " << total
                //           << std::endl;
            }
        }
    }

    return total;
}