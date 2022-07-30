// C++ test_make_appender_mutate

void test_make_appender_mutate() {
    std::vector<int> suffix{3, 4};
    auto append34 = make_appender_move(move(suffix));
    assert((std::vector<int>{1, 2, 3, 4} == append34({1, 2}))); // OK
    suffix[0] = 5;  // Undefined behavior, because suffix was moved
    assert((std::vector<int>{1, 2, 3, 4} == append34({1, 2}))); // Maybe!
}
