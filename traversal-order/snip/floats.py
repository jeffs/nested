import sys

e = sys.float_info.epsilon
numbers = [e / 3, e / 2, 1]
print(sum(numbers))       # 1.0000000000000002
print(sum(set(numbers)))  # 1.0, probably
