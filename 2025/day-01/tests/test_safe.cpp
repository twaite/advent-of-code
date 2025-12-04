#include <cassert>
#include <string>
#include <sstream>
#include <safe.h>

void test_sample_input()
{
    auto input = "L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82";
    std::istringstream ins(input);
    int result = turnDial(ins, true);

    assert(result == 3);
}

void test_sample_input_count_all_zeros()
{
    auto input = "L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82";
    std::istringstream ins(input, false);
    int result = turnDial(ins, false);

    assert(result == 6);
}

int main()
{
    test_sample_input();
    test_sample_input_count_all_zeros();
    return 0;
}
