#include <array>
#include <cassert>
#include <numeric>  // accumulate
#include <vector>

using List = std::vector<int>;

List append(List&& items, List const& suffix) {
    items.insert(items.end(), suffix.begin(), suffix.end());
    return items;
}

auto make_appender(List const& suffix) {
    return [&](List&& items) {
        return append(move(items), suffix);
    };
}

auto make_appender_move(List&& suffix) {
    return [suffix = move(suffix)](List&& items) {
        return append(move(items), suffix);
    };
}

void test_append() {
    assert((List{1, 2, 3, 4} == append({1, 2}, {3, 4})));
}

void test_append_repeatedly() {
    std::array<List, 3> suffixes = {{{8, 6}, {7, 5}, {3, 0, 9}}};
    auto const items = std::accumulate(
            suffixes.begin(), suffixes.end(),
            List{},
            append);
    assert((List{8, 6, 7, 5, 3, 0, 9} == items));
}

void test_append_reuse() {
    List items{1};
//  assert((List{1, 2} == append(items, {2})));        // Won't compile.
    assert((List{1, 2} == append(move(items), {2})));  // OK
    assert((List{1, 2} == append(move(items), {2})));  // FAIL
}

void test_make_appender() {
    List suffix{3, 4};
    auto append34 = make_appender(suffix);
    assert((List{1, 2, 3, 4} == append34({1, 2})));
}

void test_make_appender_dangle() {
    auto append34 = make_appender({3, 4});
    assert((List{1, 2, 3, 4} == append34({1, 2}))); // FAIL: Undefined behavior
}

void test_make_appender_move() {
    auto append34 = make_appender_move({3, 4});
    assert((List{1, 2, 3, 4} == append34({1, 2}))); // OK
}

// $ ./target/cpp/safety
// [1]    23388 segmentation fault  ./target/cpp/safety
void test_make_appender_mutate() {
    List suffix{3, 4};
    auto append34 = make_appender_move(move(suffix));
    assert((List{1, 2, 3, 4} == append34({1, 2}))); // OK
    suffix[0] = 5;  // Undefined behavior, because suffix was moved
    assert((List{1, 2, 3, 4} == append34({1, 2}))); // Meaningless, because UB
}

int main() {
    test_append();
    test_append_repeatedly();
    test_make_appender();
    test_make_appender_move();
}
