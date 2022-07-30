#include <array>
#include <cassert>
#include <numeric>  // accumulate
#include <vector>

std::vector<int> append(
        std::vector<int>&& items,
        std::vector<int> const& suffix) {
    items.insert(items.end(), suffix.begin(), suffix.end());
    return items;
}

auto make_appender(std::vector<int> const& suffix) {
    return [&](std::vector<int>&& items) {
        return append(move(items), suffix);
    };
}

auto make_appender_move(std::vector<int>&& suffix) {
    return [suffix = move(suffix)](std::vector<int>&& items) {
        return append(move(items), suffix);
    };
}

void test_append() {
    assert((std::vector<int>{1, 2, 3, 4} == append({1, 2}, {3, 4})));
}

void test_append_repeatedly() {
    std::array<std::vector<int>, 3> suffixes = {{{8, 6}, {7, 5}, {3, 0, 9}}};
    auto const items = std::accumulate(
            suffixes.begin(), suffixes.end(),
            std::vector<int>{},
            append);
    assert((std::vector<int>{8, 6, 7, 5, 3, 0, 9} == items));
}

void test_append_reuse() {
    std::vector<int> items{1};
//  assert((std::vector<int>{1, 2} == append(items, {2})));  // Won't compile.
    assert((std::vector<int>{1, 2} == append(move(items), {2})));  // OK
    assert((std::vector<int>{1, 2} == append(move(items), {2})));  // FAIL
}

void test_make_appender() {
    std::vector<int> suffix{3, 4};
    auto append34 = make_appender(suffix);
    assert((std::vector<int>{1, 2, 3, 4} == append34({1, 2})));
}

void test_make_appender_dangle() {
    auto append34 = make_appender({3, 4});
    assert((std::vector<int>{1, 2, 3, 4} == append34({1, 2}))); // FAIL: UB
}

void test_make_appender_move() {
    auto append34 = make_appender_move({3, 4});
    assert((std::vector<int>{1, 2, 3, 4} == append34({1, 2}))); // OK
}

// $ ./target/cpp/safety
// [1]    23388 segmentation fault  ./target/cpp/safety
void test_make_appender_mutate() {
    std::vector<int> suffix{3, 4};
    auto append34 = make_appender_move(move(suffix));
    assert((std::vector<int>{1, 2, 3, 4} == append34({1, 2}))); // OK
    suffix[0] = 5;  // Undefined behavior, because suffix was moved
    assert((std::vector<int>{1, 2, 3, 4} == append34({1, 2}))); // Maybe!
}

int main() {
    test_append();
    test_append_repeatedly();
    test_make_appender();
    test_make_appender_move();
}
