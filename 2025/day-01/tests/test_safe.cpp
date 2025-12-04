#include <cassert>
#include <sstream>
#include <string>
#include <safe.h>

void test_sample_input()
{
    std::string input = "L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82";
    int result = turnDial(input);

    assert(result == 3);
}

int main()
{
    test_sample_input();
    return 0;
}
