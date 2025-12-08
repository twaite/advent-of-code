#include <cassert>
#include <string>
#include <sstream>
#include <bank.h>

void test_total_joltage()
{
    auto input = "987654321111111\n811111111111119\n234234234234278\n818181911112111";
    std::istringstream ins(input);
    int result = total_output_joltage(ins);

    assert(result == 357);
}

int main()
{
    test_total_joltage();
    return 0;
}
