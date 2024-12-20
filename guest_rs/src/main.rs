//! This is the main file of the program. It contains the user-defined functions that need to be made available to the VM.
//! The `export_fn!` macro is used to register the exported functions with the VM. The string literal is the exported name of the function as visible from outside the VM.
//! Everything runs in a `no_std` environment but dynamic memory allocation is available.

#![no_std]
#![no_main]

mod rv;
// Do not edit above this line

// custom alloc here
use alloc::vec::Vec;

// Add your code below
use blake2::{Blake2s256, Digest};

fn compute_hash(args: &[u8]) -> Vec<u8> {
    Blake2s256::digest(args).to_vec()
}

fn add(args: &[u8]) -> Vec<u8> {
    if args.len() != 2 {
        panic!("Invalid arguments");
    }
    match args[0].checked_add(args[1]) {
        Some(v) => [v].to_vec(),
        None => panic!("The sum of the operands overflows u8"),
    }
}

fn prepend_hello(args: &[u8]) -> Vec<u8> {
    let mut res = Vec::new();
    res.extend_from_slice(b"hello ");
    res.extend_from_slice(args);
    res
}

fn compute_dot(args: &[u8]) -> Vec<u8> {
    // Ensure correct input length (two 128-dimensional vectors, 4 bytes per f32)
    assert_eq!(args.len(), 1024); // 128 * 4 * 2

    // Convert byte slice to f32 vectors
    let mut vec1 = Vec::with_capacity(128);
    let mut vec2 = Vec::with_capacity(128);

    // Process first vector
    for chunk in args[..512].chunks_exact(4) {
        let value = f32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]);
        vec1.push(value);
    }

    // Process second vector
    for chunk in args[512..].chunks_exact(4) {
        let value = f32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]);
        vec2.push(value);
    }

    // Calculate dot product
    let dot_product: f32 = vec1.iter()
        .zip(vec2.iter())
        .map(|(&a, &b)| a * b)
        .sum();

    // Convert result back to byte array
    dot_product.to_le_bytes().to_vec()
}

// export your functions here
export_fn!(
    "compute_hash" => compute_hash,
    "add" => add,
    "prepend_hello" => prepend_hello,
    "compute_dot" => compute_dot
);
