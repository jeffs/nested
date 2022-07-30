C++ has undefined behavior.

    $ make test
    c++ -std=c++20 -pedantic -Wall -Wextra src/safety.cpp -o target/cpp/safety
    target/cpp/safety
    safety: src/safety.cpp:53: void test_make_appender_dangle(): Assertion `(std::vector<int>{1, 2, 3, 4} == append34({1, 2}))' failed.
    make: *** [Makefile:12: test_safety] Aborted
