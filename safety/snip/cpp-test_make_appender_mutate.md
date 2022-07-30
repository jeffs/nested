C++ undefined behavior can cause segfaults.

    $ make test
    c++ -std=c++20 -pedantic -Wall -Wextra src/safety.cpp -o target/cpp/safety
    target/cpp/safety
    make: *** [Makefile:12: test_safety] Segmentation fault
