// 1. References are a kind of pointers to borrow the value they point to.
// 2. Smart pointers are data structures that have additional metadata and capabilities.
// 3. Smart pointers have a counter to count references and ownership of the value.
// 4. String and Vec<T> are smart pointers.
// 5. Smart pointers implement the Deref and Drop traits.
// 6. The Deref trait allows an instance to behave like a reference.
// 7. The Drop trait indicates how to destroy an instance goes out of scope.

pub fn pointer1() {
    println!("pointer1");
}