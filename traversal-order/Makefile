CXX = c++
CXXFLAGS = -std=c++20 -pedantic -Wall -Wextra

.PHONY: all clean run

all: target/cpp/floats target/cpp/words

run: target/cpp/floats target/cpp/words
	target/cpp/floats
	target/cpp/words
	

clean:
	rm -fr target/cpp

target/cpp/%: src/%.cpp
	@mkdir -p target/cpp
	$(CXX) $(CXXFLAGS) $< -o $@
