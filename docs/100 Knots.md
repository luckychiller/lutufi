


> *"Trust in Allah, then tie your camel."*

To build **Lutufi**—a high-performance, mathematically rigorous causal inference engine with Python bindings—you cannot write "Python in Rust." You must write idiomatic, memory-safe, concurrent Rust. 

Building graphs and probabilistic models in Rust is notoriously difficult for beginners because Rust’s ownership model strictly prohibits the kind of willy-nilly circular referencing that is easy in Python or C++.

Here is your **100-Project Rust Ascendancy Roadmap**. It is divided into 10 Phases. The first 50 projects teach you Rust. The next 30 teach you the computer science needed for Lutufi. The final 20 are the direct pre-flight components of Lutufi itself.

---

### Phase 1: Syntax, Borrowing, and the Compiler (Projects 1-10)
*Goal: Stop fighting the borrow checker and understand basic memory ownership.*

1. **Hello World & Cargo:** Initialize a project, understand `Cargo.toml`.
2. **Temperature Converter:** Variables, mutability, basic math, and standard I/O.
3. **Guessing Game:** `match` statements, `loop`, handling basic `Result` types.
4. **Nth Fibonacci (Iterative vs Recursive):** Functions, basic stack memory, integer types.
5. **String Reverser:** Understand the difference between `String` (owned) and `&str` (borrowed).
6. **Word Frequency Counter:** Intro to `HashMap` and iterating over text.
7. **Vector Statistics (Mean, Median, Mode):** Sorting `Vec<T>`, handling empty vectors with `Option<T>`.
8. **Struct Basics (User Profile):** Defining `struct`, using `impl` blocks for methods.
9. **Enum Basics (Traffic Light):** Defining `enum`, exhaustive `match` branching.
10. **Simple CLI To-Do List:** Reading/writing to a text file using `std::fs`.

### Phase 2: Traits, Generics, & Error Handling (Projects 11-20)
*Goal: Write reusable code and learn to use `Result` instead of crashing.*

11. **Generic Min/Max Function:** Using `<T: PartialOrd>`.
12. **Shape Area Calculator:** Defining a `Trait` (Shape) and implementing it for Structs (Circle, Rectangle).
13. **Custom `Display` and `Debug`:** Implementing standard traits for your own structs.
14. **Safe Integer Divider:** Returning custom `Result<f64, DivisionError>`.
15. **Lutufi Prep: Custom Error Hierarchy:** Create an enum `LutufiError` with `CyclicGraph`, `InvalidCPT`, etc., using the `thiserror` crate.
16. **Config File Parser (JSON):** Intro to the `serde` and `serde_json` crates.
17. **CSV Reader:** Parsing structured data into a `Vec<Struct>` using the `csv` crate.
18. **Environment Variable Manager:** Reading `env::args` and `std::env::vars`.
19. **Trait Objects:** Create a `Vec<Box<dyn Shape>>` to understand dynamic dispatch.
20. **Lutufi Prep: The Builder Pattern:** Create a `BayesianNetworkBuilder` struct that chains methods (`.add_node()`, `.add_edge()`) and consumes itself in `.build()`.

### Phase 3: Iterators, Closures, & Functional Rust (Projects 21-30)
*Goal: Write fast, zero-cost data pipelines without `for` loops.*

21. **Custom Iterator:** Implement the `Iterator` trait for a custom Fibonacci sequence struct.
22. **Map/Filter/Fold Pipeline:** Process a massive CSV of numbers using iterator chaining.
23. **Closure Sandbox:** Pass closures (lambdas) as arguments to functions (`Fn`, `FnMut`, `FnOnce`).
24. **Event Listener:** Store a list of callbacks (`Box<dyn Fn()>`) in a struct and trigger them.
25. **Prime Number Sieve:** Efficient data manipulation using iterators and `Vec`.
26. **Matrix Transposer:** 2D vectors (`Vec<Vec<T>>`) and manipulating row/column data.
27. **Memoization Cache:** Use a `HashMap` and a closure to cache expensive function calls.
28. **Lazy Evaluator:** Store a computation as a closure and only execute it when `.value()` is called.
29. **Text Search (grep clone):** Search for a string in a file using `filter`.
30. **Lutufi Prep: Log-Space Math Module:** Implement addition and multiplication in log-space (the log-sum-exp trick) with comprehensive unit tests.

### Phase 4: Lifetimes & Advanced Memory (Projects 31-40)
*Goal: Mastering references. If you fail this, you cannot build graphs in Rust.*

31. **String Slicer:** A function that takes a string and returns a struct containing `&str` slices with explicit lifetimes (`'a`).
32. **Config Reference Holder:** A struct that borrows heavily from a loaded config file without copying it.
33. **The Boxed Linked List:** Implement a Singly Linked List using `Box<Node>`.
34. **The `Rc` Shared Pointer:** Share ownership of a piece of data across multiple variables using `Rc<T>`.
35. **The `RefCell` Mutability:** Mutate data through an immutable reference using interior mutability.
36. **The `Rc<RefCell<T>>` Tree:** Build a simple Tree data structure where nodes have multiple parents/children. *(Warning: You will feel the pain of Rust graphs here).*
37. **Arena Allocator Graph:** Re-build the Tree, but store all nodes in a flat `Vec<Node>` and use `usize` indices to represent edges. **(Crucial for Lutufi's graph architecture).**
38. **Sparse Matrix (COO):** Create a Coordinate format sparse matrix for memory efficiency.
39. **Sparse Matrix (CSR):** Convert COO to Compressed Sparse Row for fast lookups.
40. **Lutufi Prep: The `Domain` & `Variable` Structs:** Implement the Phase 1.1 Variable/Assignment data structures exactly as described in your roadmap.

### Phase 5: Concurrency & Parallelism (Projects 41-50)
*Goal: Multi-threading for Phase 10 (Scalability).*

41. **Thread Spawner:** Spawn 10 threads, do math, and `join()` them.
42. **Message Passing (MPSC):** Send data from worker threads to a main aggregator thread.
43. **Shared State (`Arc<Mutex<T>>`):** Safely mutate a shared counter from multiple threads.
44. **Deadlock Creator:** Intentionally cause a deadlock to understand Mutex lock ordering.
45. **Thread Pool:** Build a custom thread pool for executing arbitrary jobs.
46. **Rayon Intro:** Use the `rayon` crate to parallelize a `.iter().map()` pipeline.
47. **Parallel Matrix Multiplication:** Implement Matrix Math chunked across multiple threads.
48. **Async Basics:** Fetch data from 3 APIs concurrently using `tokio` and `reqwest`.
49. **Async File I/O:** Read/write large files asynchronously.
50. **Lutufi Prep: Parallel Factor Product:** Use `rayon` to multiply multiple large probability tables in parallel.

### Phase 6: Graph Theory & Algorithms (Projects 51-60)
*Goal: The mathematical and structural foundation of Lutufi.*

51. **Adjacency List Graph:** Implement a generic directed graph using the Arena (Vec index) method.
52. **Depth First Search (DFS):** Traverse the graph.
53. **Breadth First Search (BFS):** Find shortest paths in an unweighted graph.
54. **Topological Sort:** Order nodes such that all parents come before children.
55. **Cycle Detector:** Detect if a graph contains a cycle (Crucial for `LutufiCyclicGraphError`).
56. **Markov Blanket Finder:** Given a node, find its parents, children, and children's parents.
57. **Graph Serialization:** Serialize your graph to a JSON string and back using Serde.
58. **Connected Components:** Find all isolated subgraphs in an undirected graph.
59. **Dijkstra’s Algorithm:** Shortest path on a weighted graph.
60. **Lutufi Prep: The Bayesian Network Struct:** Combine Variable, Domain, and your Arena Graph into a safe, acyclic `BayesianNetwork` wrapper.

### Phase 7: FFI & Python Bindings (`PyO3`) (Projects 61-70)
*Goal: Making Rust usable for Data Scientists.*

61. **PyO3 Hello World:** Expose a basic Rust function to Python.
62. **PyO3 Number Cruncher:** Pass a list of numbers from Python, compute the mean in Rust, return it.
63. **PyO3 Struct Wrapper:** Create a Rust struct, annotate it with `#[pyclass]`, and expose methods to Python.
64. **PyO3 Error Translation:** Catch Rust `Result::Err` and raise a native Python `Exception`.
65. **Numpy to Rust (`rust-numpy`):** Pass a numpy array to Rust, mutate it, and pass it back.
66. **Python Builder Pattern:** Expose your Rust Builder Pattern (Project 20) to Python beautifully.
67. **Maturin Build:** Use `maturin` to build a `.whl` file of your Rust/Python library.
68. **Pandas Integration:** Accept a Pandas DataFrame in Python, extract columns, and pass vectors to Rust.
69. **Async PyO3:** Expose an async Rust function to Python's `asyncio`.
70. **Lutufi Prep: The Phase 1.4 API:** Build the exact Python API string: `BayesianNetwork.builder().add_variable(...).build()`.

### Phase 8: Probabilistic Math & Factors (Projects 71-80)
*Goal: The core data structures for Exact Inference.*

71. **The `Assignment` Struct:** A mapping of Variable IDs to their current integer states.
72. **The `Factor` Trait:** Define `scope()`, `evaluate()`, `multiply()`, and `marginalize()`.
73. **Dense Multidimensional Array:** Implement a 1D vector acting as an N-dimensional tensor (stride math).
74. **`TabularFactor` Implementation:** Back the `Factor` trait with your dense array.
75. **Factor Multiplication:** Given two factors sharing variables, correctly compute the product.
76. **Factor Marginalization:** Sum out a variable from a factor.
77. **Factor Normalization:** Ensure a factor sums to 1.0.
78. **CPT Validation:** Ensure that for every parent configuration, the child probabilities sum to 1.0.
79. **Noisy-OR Factor:** Implement the parameterized version of a CPT.
80. **Lutufi Prep: The Asia Network Hardcode:** Manually construct the factors for the Asia network and multiply them out.

### Phase 9: Testing, CI/CD, & Benchmarking (Projects 81-90)
*Goal: Project infrastructure to ensure mathematical honesty.*

81. **Cargo Tests:** Unit tests, integration tests, and `#[should_panic]` tests.
82. **Pytest Integration:** Set up Pytest to test your PyO3 compiled module.
83. **Criterion Benchmarks:** Benchmark two different matrix multiplication strategies in Rust.
84. **Floating Point Tolerance Tests:** Write macros to assert equality within `1e-10`.
85. **Code Coverage:** Set up `tarpaulin` to measure Rust code coverage.
86. **GitHub Actions CI:** Trigger cargo build and cargo test on every push.
87. **Automated Wheels:** Set up `cibuildwheel` to build your Python library for Mac/Windows/Linux.
88. **Fuzz Testing:** Set up `cargo-fuzz` and throw garbage at your Graph builder.
89. **Documentation (Rustdoc):** Document your code properly and run `cargo doc`.
90. **Lutufi Prep: The Ground Truth Runner:** Implement the exact JSON ground truth infrastructure defined in Sprint 0.2.

### Phase 10: The Lutufi Pre-Flight (Projects 91-100)
*Goal: Building the direct prototypes of Phase 2 (Exact Inference).*

91. **Variable Elimination (VE) - Part 1:** Given a query and evidence, reduce the factors.
92. **Variable Elimination - Part 2:** Implement Min-Fill and Min-Degree heuristics.
93. **Moralization:** Convert a directed graph to an undirected moral graph.
94. **Triangulation:** Add fill-in edges to make the moral graph chordal.
95. **Maximum Cardinality Search:** Find the cliques in the chordal graph.
96. **Junction Tree Construction:** Build the tree of cliques and verify the Running Intersection Property.
97. **Belief Propagation (Mock):** Pass sum-product messages across a simple 3-node chain.
98. **D-Separation:** Implement the Bayes Ball algorithm to test independence.
99. **LMF Serialization:** Finalize your custom JSON schema for saving/loading networks.
100. **The First Commit:** Gather everything you've learned. Set up the repo exactly as described in Phase 0. Write the `LANGUAGE_DECISION.md`. Begin Sprint 1.1.

---

### How to execute this roadmap:

1. **Do not copy-paste.** Type every line.
2. **Embrace the compiler.** The Rust compiler (`rustc`) and linter (`clippy`) are your senior engineers. When they yell at you, read the error message. It tells you exactly how to fix it.
3. **Use the Book.** Read *The Rust Programming Language* (free online) alongside Phases 1-4.
4. **Graph memory matters.** By Project 37, you will realize why Rust graphs are hard. *Do not use pointers for your graphs.* Use indices in a `Vec` (Arena allocation). This is how professional high-performance graphs (like `petgraph`) are built in Rust.

You have 100 knots to tie before you build the camel. Start at Project 1.