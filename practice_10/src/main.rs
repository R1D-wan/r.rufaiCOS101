fn main() {
    let v: Vec<i32> = vec![42, 30, 40, 100];
    // Vector v is allocated in the heap
    // Only 1 single variable owns the heap memory at any given time
    let  v2 = v;
    // there two variables own heap values
    // two pointers to the same content is not allowed in Rust
    // But it is very smart in terms of memory access, so it detects a race condition
    // v2 variable will point to same heap

    println!("{:?}", v);
}