void test_make_appender_dangle() {
    auto append34 = make_appender({3, 4});
    assert((std::vector<int>{1, 2, 3, 4} == append34({1, 2}))); // FAIL: UB
}
