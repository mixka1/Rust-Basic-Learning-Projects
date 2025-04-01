//Rust based AES encrypter and decrypter
//Author: Kaiden Mix
//Purpose: This Rust program allows the user to enter a message, encrypt it using AES-256-CBC,
// and then decrypt it back to verify correctness. It utilizes random key and IV generation
// for security.

use aes::Aes256;
use cipher::{BlockCipher, BlockEncrypt, BlockDecrypt, KeyInit};
use cipher::generic_array::GenericArray;
use rand::Rng;
use std::io::{self, Write};

//Generates a random 32 character password
fn generate_key() -> [u8; 32] {
    rand::thread_rng().r#gen::<[u8; 32]>()
}

//Generates a random 16 character starting point
fn generate_iv() -> [u8; 16] {
    rand::thread_rng().r#gen::<[u8; 16]>()
}

//Function for encryption, takes our message, the key, and starting point.
fn encrypt(plaintext: &str, key: &[u8; 32], iv: &[u8; 16]) -> Vec<u8> {
    let cipher = Aes256::new(GenericArray::from_slice(key));
    let mut buffer = plaintext.as_bytes().to_vec();
    
    // Add PKCS7 padding
    let pad_len = 16 - (buffer.len() % 16);
    buffer.extend(vec![pad_len as u8; pad_len]);
    
    let mut encrypted = Vec::new();
    let mut previous_block = GenericArray::clone_from_slice(iv);
    
    for chunk in buffer.chunks(16) {
        let mut block = GenericArray::clone_from_slice(chunk);
        
        // XOR with previous block/IV before encryption
        for (b, p) in block.iter_mut().zip(previous_block.iter()) {
            *b ^= *p;
        }
        
        cipher.encrypt_block(&mut block);
        encrypted.extend_from_slice(block.as_slice());
        previous_block = block;
    }
    
    encrypted
}

//Function for decryption, takes the scrambled message, key, and starting point used.
fn decrypt(ciphertext: &[u8], key: &[u8; 32], iv: &[u8; 16]) -> Result<String, Box<dyn std::error::Error>> {
    let cipher = Aes256::new(GenericArray::from_slice(key));
    let mut decrypted = Vec::new();
    let mut previous_block = GenericArray::clone_from_slice(iv);
    
    for chunk in ciphertext.chunks(16) {
        let mut block = GenericArray::clone_from_slice(chunk);
        let encrypted_block = block.clone();
        
        cipher.decrypt_block(&mut block);
        
        // XOR with previous block/IV after decryption
        for (b, p) in block.iter_mut().zip(previous_block.iter()) {
            *b ^= *p;
        }
        
        decrypted.extend_from_slice(block.as_slice());
        previous_block = encrypted_block;
    }
    
    // Remove PKCS7 padding
    let pad_len = *decrypted.last().ok_or("Empty ciphertext")?;
    decrypted.truncate(decrypted.len() - pad_len as usize);
    
    Ok(String::from_utf8(decrypted)?)
}

fn main() {
    let key = generate_key(); 
    let iv = generate_iv();
    print!("Enter a message to encrypt: ");
    io::stdout().flush().unwrap(); //Message to encrypt
    
    let mut plaintext = String::new();
    io::stdin().read_line(&mut plaintext).unwrap();
    let plaintext = plaintext.trim();

    //Display plaintext as well as our key and starting point
    println!("Original text: {}", plaintext);
    println!("Key: {}", hex::encode(key));
    println!("IV: {}", hex::encode(iv));
    
    //Encrypts
    let ciphertext = encrypt(plaintext, &key, &iv);
    println!("Encrypted (hex): {}", hex::encode(&ciphertext));
    
    //Decrypts
    match decrypt(&ciphertext, &key, &iv) {
        Ok(decrypted) => println!("Decrypted: {}", decrypted),
        Err(e) => println!("Decryption error: {}", e),
    }
}