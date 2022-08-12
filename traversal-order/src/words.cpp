#include <cassert>
#include <unordered_set>
#include <vector>

int main() {
    std::vector words{"Hello", "World"};
    std::unordered_set set(words.begin(), words.end());
    std::vector vec(set.begin(), set.end());
    assert((words == vec)); // Who knows?
    return 0;
}
