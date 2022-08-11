import sys

epsilon = sys.float_info.epsilon
ordered = [epsilon / 3, epsilon / 2, 1]
unordered = set(ordered)
print(sum(ordered))   # 1.0000000000000002
print(sum(unordered)) # 1.0

sys.exit(0)

from math import ulp

delta = ulp(1) / 2  # Half the value of the least significant bit of 1.0.
ordered = (delta, delta, 1)
assert sum(ordered) != sum(reversed(values))  # 1.0 != 1.0000000000000002

s, t = "Hello", "World"
assert s + t != t + s  # "HelloWorld" != "WorldHello"


from functools import reduce
from operator import add


def my_sum(head, *tail):
    """Adds each item in tail to head."""
    return reduce(add, tail, head)


assert my_sum("Hello", "World") != my_sum("World", "Hello")

# Ideally, operations on sets would recognized associativity, and parallelize
# where possible; and moreover, would automatically diagnose any dependence on
# the order of a semantically unordered collection.  In fact, any set of tasks
# would ideally have a dependency graph that could be used to parallelize
# operations to whatever extent makes sense.
