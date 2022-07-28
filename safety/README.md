See also: ../../lazy-evaluation/oops.cpp

Can you figure out why main.cpp is broken?
Can you spot the difference in ok.cpp?

The difference is that in main.cpp, we did not explicitly move the original
vector.  We captured it by reference, and the reference dangled.  Note that
even a modern C++ compiler with warnings cranked up does not detect this issue;
so don't feel bad if you didn't notice it, either.  Note also that passing
function arguments by value instead of by reference would not help:  The
capture, not the parameter passing, is the problem.

The problem here is that C++ is inherently not memory safe.  What happens if we
try it in Rust?  Compile-time error, again fixed by explicitly moving the
parameter into the closure.  See main.rs and ok.rs.

## Go
On the one hand, Go can't really express any of this.  On the other hand, being
garbage collected, it doesn't have to.  Go doesn't even need a vector or
ArrayList type, because GC makes raw arrays so useful.

As a Go programmer, there are things you cannot do; and that's actually really
cognitively freeing, and can wonderfully improve both learning curve and
productivity.  However, note some caveats.

In Go, you either copy each parameter slice's underlying array elements, or you
share a reference to them with the calling scope.  The former approach is
potentially inefficient, while the latter is error-prone.  Even in the former
case, it's easy to forget to make the copy; it relies on programmer discipline
that belies the apparently low learning curve of Go.  As I find that debugging
and is the greatest pain point in software maintenance, I still prefer Rust.
