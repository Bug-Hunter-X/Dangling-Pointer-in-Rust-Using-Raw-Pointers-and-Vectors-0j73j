fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);

    let ptr = vec.as_ptr();

    // Reassign vec, deallocating the memory it previously held
    vec = Vec::new();

    // Use the dangling pointer! This will lead to undefined behavior
    println!("Value at dangling pointer: {}", unsafe { *ptr });
}