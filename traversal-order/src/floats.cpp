#include <iostream> // cout
#include <limits>   // numeric_limits
#include <numeric>  // accumulate
#include <unordered_set>
#include <vector>

template<class C>
void print_sum(C const& values) {
    auto const sum = std::accumulate(values.begin(), values.end(), 0.0);
    std::cout << sum << '\n';
}

int main() {
    std::cout.precision(20);
    auto const epsilon = std::numeric_limits<double>::epsilon();
    std::vector<double> ordered{epsilon / 3, epsilon / 2, 1};
    std::unordered_set<double> unordered(ordered.begin(), ordered.end());
    print_sum(ordered);     // 1.000000000000000222
    print_sum(unordered);   // 1
    return 0;
}
