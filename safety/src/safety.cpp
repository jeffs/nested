#include <array>
#include <cassert>
#include <numeric>  // accumulate
#include <vector>

using List = std::vector<int>;

List concatenate(List&& items, List const& suffix) {
    items.insert(items.end(), suffix.begin(), suffix.end());
    return items;
}

void test_concatenate() {
    assert((List{1, 2, 3, 4} == concatenate({1, 2}, {3, 4})));
}

void test_concatenate_repeatedly() {
    std::array<List, 3> suffixes = {{{8, 6}, {7, 5}, {3, 0, 9}}};
    auto const items = std::accumulate(
            suffixes.begin(), suffixes.end(),
            List{},
            concatenate);
    assert((List{8, 6, 7, 5, 3, 0, 9} == items));
}

void test_concatenate_reuse() {
    List items{1};
//  assert((List{1, 2} == concatenate(items, {2})));        // Won't compile.
    assert((List{1, 2} == concatenate(move(items), {2})));  // OK
//  assert((List{1, 2} == concatenate(move(items), {2})));  // Failure!
}

int main() {
    test_concatenate();
    test_concatenate_repeatedly();
    test_concatenate_reuse();
}
