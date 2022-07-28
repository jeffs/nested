#include <cassert>
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

int main() {
    test_concatenate();
}
