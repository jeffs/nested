#include <array>
#include <cassert>
#include <numeric>  // accumulate
#include <vector>

std::vector<int> concatenate(
        std::vector<int>&& items,
        std::vector<int> const& suffix) {
    items.insert(items.end(), suffix.begin(), suffix.end());
    return items;
}

void test_concatenate() {
    assert((std::vector<int>{1, 2, 3, 4} == concatenate({1, 2}, {3, 4})));
}

void test_concatenate_repeatedly() {
    std::array<std::vector<int>, 3> suffixes = {{{8, 6}, {7, 5}, {3, 0, 9}}};
    auto const items = std::accumulate(
            suffixes.begin(), suffixes.end(),
            std::vector<int>{}, concatenate);
    assert((std::vector<int>{8, 6, 7, 5, 3, 0, 9} == items));
}

int main() {
    test_concatenate();
    test_concatenate_repeatedly();
}
