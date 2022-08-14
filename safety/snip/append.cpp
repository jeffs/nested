std::vector<int> append(
        std::vector<int>&& items,
        std::vector<int> const& suffix) {
    items.insert(items.end(), suffix.begin(), suffix.end());
    return items;
}

void test_append() {
    assert((std::vector<int>{1, 2, 3, 4} == append({1, 2}, {3, 4})));
}
