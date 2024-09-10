# File Encryptor/Decryptor

## Overview

This project implements a file encryption and decryption tool using the AES algorithm. It allows users to encrypt and decrypt files with a specified password.

## Purpose

The goal is to provide functionality to:
1. Encrypt a file using AES encryption.
2. Save the encrypted file.
3. Decrypt the encrypted file when needed.

## Libraries Used

- `aes`: For AES encryption and decryption.
- `rand`: For generating random values (if needed in future enhancements).
- `base64`: For encoding/decoding the data (if needed in future enhancements).

## Functionality

1. **File Reading and Encryption**: Reads the content of a file and encrypts it using AES.
2. **Save Encrypted File**: Saves the encrypted data to a new file.
3. **Decryption**: Decrypts the file content back to its original form.

## Algorithm

1. Read the file specified by the user.
2. Encrypt the file content using AES with the given key.
3. Save the encrypted data to a new file.
4. Implement decryption to restore the original file content.

## Code Explanation

- **Key Definition**: Defines a 128-bit key (16 bytes) for AES encryption.
- **Read File**: Reads the content of the file `matn.txt`.
- **Encryption**: Creates an AES128 instance and encrypts a 16-byte block of data.
- **Save Encrypted File**: Writes the encrypted data to `shifrtlangan.tex`.
- **Decryption**: Decrypts the data to verify the encryption and prints the result.

## Usage

- Ensure you have the necessary dependencies in your `Cargo.toml`.
- Place the file `matn.txt` in the project directory.
- Run the program to see the encryption and decryption process.

## Example

```rust
use std::{error::Error, fs};
use aes::Aes128;
use cipher::{generic_array::GenericArray, BlockDecrypt, BlockEncrypt, KeyInit};

fn main() -> Result<(), Box<dyn Error>> {
    let key = b"qwertyuiopasd123"; // 128-bit key (16 bytes)
    let data = fs::read("matn.txt")?; // Read file content
    let encryptor: Aes128 = Aes128::new(GenericArray::from_slice(key)); // Create encryption object

    let mut block = GenericArray::clone_from_slice(&data[..16]); // Process 16 bytes at a time
    encryptor.encrypt_block(&mut block); // Encrypt data
    fs::write("encrypted.txt", &block)?; // Save encrypted data to new file

    let mut decrypted_block = block.clone(); // Clone encrypted block
    encryptor.decrypt_block(&mut decrypted_block); // Decrypt block
    println!("{:?}", decrypted_block); // Print decrypted data

    Ok(())
}
