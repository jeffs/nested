CXX = c++
CXXFLAGS = -std=c++20 -pedantic -Wall -Wextra

.PHONY: all clean test

all: target/cpp/safety

clean:
	rm -fr target/cpp

test_safety: target/cpp/safety
	$<

test: test_safety

target/cpp/%: src/%.cpp
	@mkdir -p target/cpp
	$(CXX) $(CXXFLAGS) $< -o $@
