CXX = c++
CXXFLAGS = -std=c++20 -pedantic -Wall -Wextra

.PHONY: all
all: target/ok target/main

target/main: src/main.cpp
	@mkdir -p target
	$(CXX) $(CXXFLAGS) $< -o $@


target/ok: src/ok.cpp
	@mkdir -p target
	$(CXX) $(CXXFLAGS) $< -o $@


.PHONY: run
run: target/ok target/main
	target/ok
	target/main
