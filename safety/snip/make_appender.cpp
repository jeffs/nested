// C++ make_appender

auto make_appender(std::vector<int> const& suffix) {
    return [&](std::vector<int>&& items) {
        return append(move(items), suffix);
    };
}

void test_make_appender() {
    std::vector<int> suffix{3, 4};
    auto append34 = make_appender(suffix);
    assert((std::vector<int>{1, 2, 3, 4} == append34({1, 2})));
}
