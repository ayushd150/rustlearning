rust has 2 steps

build/optimise  => cargo build - rust to binary

and then run the file

numbers:-

i8 - i(signed integer) and size [rust is memory safe]

memory management

garbage collector(no dangling pointers, cant do manual memory managaement)
manual
rust way

1) Mutability - immutable var represent var whose value cannot be chnaged once assigned

2) stack vs heap

3)  ownership - set of rules that govern how rust manage memory


Rust: Structs

Definition: A struct in Rust is a custom data type that groups related fields together.

Static typing: All fields must have explicitly defined types.

Immutable by default: Once created, values are immutable unless declared mut.

Memory-safe: Enforced at compile-time, no nulls unless you use Option<T>.

JavaScript: Objects

Definition: JS doesn’t have structs; instead, it has objects, which are flexible collections of key–value pairs.

Dynamic typing: Fields don’t need declared types.

Mutable by default: You can freely add, remove, or change properties at runtime.

Loose structure: Any property can be added on the fly.

Strings vs slices
Strings(mutable) - growable, heap-allocated, owned string type, comes from standard ibrary, size not fixed at compile time

Slices - immutable refrence to portion of string
Key traits of &str:

Borrowed view (no ownership).

Immutable (cannot change content).

Fixed size at compile-time (just a pointer + length).