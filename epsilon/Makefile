CXX = c++
CXXFLAGS = -std=c++20 -pedantic -Wall -Wextra

.PHONY: all clean run

all: target/cpp/main

run: target/cpp/main
	$<

clean:
	rm -fr target/cpp

target/cpp/%: src/%.cpp
	@mkdir -p target/cpp
	$(CXX) $(CXXFLAGS) $< -o $@
