#include <cassert>
#include <string>
#include <sstream>
#include <safe.h>

void test_sample_input()
{
    auto input = "L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82";
    std::istringstream ins(input);
    int result = turnDial(ins);

    assert(result == 3);
}

int main()
{
    test_sample_input();
    return 0;
}
