use std::io::Write;

// Hardcoded Dictionary mapping for 'transfer(address,uint256)'
const TRANSFER_SELECTOR: &[u8; 4] = &[0xa9, 0x05, 0x9c, 0xbb];
const TRANSFER_DICT_ID: u8 = 0x01;

/// Zero-copy dictionary substitution and Zstd compression.
/// Expects raw transaction payload bytes intercepted from the RPC mempool.
pub async fn pack_transaction(payload: &[u8]) -> Result<Vec<u8>, std::io::Error> {
    if payload.len() < 4 {
        return Ok(payload.to_vec()); // Payload too small, skip compression
    }

    // 1. Dictionary Substitution (Zero-copy slice inspection)
    let (selector, remaining_payload) = payload.split_at(4);
    
    // Pre-allocate buffer to prevent heap reallocation pauses
    let mut intermediate_buffer = Vec::with_capacity(payload.len());

    if selector == TRANSFER_SELECTOR {
        intermediate_buffer.push(TRANSFER_DICT_ID);
        // Extend from slice allows bulk memory copying (memcpy under the hood)
        intermediate_buffer.extend_from_slice(remaining_payload); 
    } else {
        intermediate_buffer.push(0x00); // 0x00 indicates no dictionary match
        intermediate_buffer.extend_from_slice(payload);
    }

    // For the Hackathon EVM/Yul demo, we only send the dictionary-packed bytes.
    // Full Zstd decompression on-chain requires a custom precompile or off-chain coprocessor.
    // The Yul gateway is currently hardcoded to expand the dictionary ID.

    // Final payload ready to be injected into Monad
    Ok(intermediate_buffer)
}
