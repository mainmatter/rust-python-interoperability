# Wrapping up

You should now have a strong theoretical foundation to reason about your options whenever
you have a Python problem that could benefit from a concurrent solution.

That's only the beginning, though. To truly master conccurent programming, you need to practice it!\
This wasn't the right venue to cover all the possible concurrency patterns you can express with Rust.
Luckily enough, there's plenty of resources available to help you on that side! We recommend, in particular,
the ["Threads" chapter](https://rust-exercises.com/100-exercises/07_threads/00_intro.html) of our own
Rust course. It follows the same hands-on approach we've been using in this book, and it's a great way to
get more practice with Rust's concurrency primitives.

Take the final exercise as a capstone project: you'll have to design a non-trivial algorithm and piece together
various concurrenty primitives to implement a solution that's both correct and efficient. Don't shy
away from the challenge: embrace it, it's the best way to learn!

## Beyond performance

A note, in closing: writing **correct** concurrent code is tricky.\
We highlighted Rust, in this chapter, as a way to circumvent the limitations of Python's GIL and ultimately
improve the performance of your code. But that's only half of the story.\
Rust's type system and ownership model make it much easier to write concurrent code that's correct, too.
As you delve deeper into the world of concurrent programming, you'll come to appreciate the real value of Rust's
`Send` and `Sync` traits, which we've only briefly touched upon when discussing data races.

As the saying goes: people come to Rust for the performance, but they stay for its safety and correctness guarantees.
