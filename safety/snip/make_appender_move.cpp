// C++ make_appender_move

auto make_appender_move(std::vector<int>&& suffix) {
    return [suffix = move(suffix)](std::vector<int>&& items) {
        return append(move(items), suffix);
    };
}

void test_make_appender_move() {
    auto append34 = make_appender_move({3, 4});
    assert((std::vector<int>{1, 2, 3, 4} == append34({1, 2}))); // OK
}

