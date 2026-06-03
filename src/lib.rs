use hex::{decode, encode};

pub fn decode_hex(hex_str: &str) -> Result<Vec<u8>, String> {
    // Decode hex string into Vec<u8>, return error string on failure
     hex::decode(hex_str).map_err(|err| err.to_string())
}

pub fn to_big_endian(bytes: &[u8]) -> Vec<u8> {
    // Reverse the byte order of input slice and return as Vec<u8>
    let reversed: Vec<u8> = bytes.iter().rev().copied().collect();
    reversed
}
pub fn bytes_to_hex(bytes: &[u8]) -> String {
    // Implement conversion of bytes slice to hex string
    hex::encode(bytes).to_string()
}

pub fn hex_to_bytes(hex: &str) -> Result<Vec<u8>, hex::FromHexError> {
    // Implement conversion of hex string to bytes vector
      hex::decode(hex)
}

pub fn swap_endian_u32(num: u32) -> [u8; 4] {
    // Implement little-endian byte swap for u32
    u32::to_le_bytes(num)
}

pub fn parse_satoshis(input: &str) -> Result<u64, String> {
    // Parse input string to u64, return error string if invalid
    let res = input.parse::<u64>().map_err(|e| e.to_string());
    res
}

pub enum ScriptType {
    P2PKH,
    P2WPKH,
    Unknown,
}

pub fn classify_script(script: &[u8]) -> ScriptType {
    // Match script pattern and return corresponding ScriptType
    if script[0] == 0x76u8 && script[1] == 0xa9u8 {
        ScriptType::P2PKH
    } else if script[0] == 0xa9u8 {
        ScriptType::P2WPKH
    } else if script.len() == 0 {
        ScriptType::Unknown
    } else {
        ScriptType::Unknown
    }
}

// TODO: complete Outpoint tuple struct
pub struct Outpoint();

pub fn read_pushdata(script: &[u8]) -> &[u8] {
    // TODO: Return the pushdata portion of the script slice (assumes pushdata starts at index 2)
}

pub trait Wallet {
    fn balance(&self) -> u64;
}

pub struct TestWallet {
    pub confirmed: u64,
}

impl Wallet for TestWallet {
    fn balance(&self) -> u64 {
        // TODO: Return the wallet's confirmed balance
    }
}

pub fn apply_fee(balance: &mut u64, fee: u64) {
    // TODO: Subtract fee from mutable balance reference
}

pub fn move_txid(txid: String) -> String {
    // TODO: Return formatted string including the txid for display or logging
}

// TODO: Add necessary derive traits
pub enum Opcode {
    OpChecksig,
    OpDup,
    OpInvalid,
}

impl Opcode {
    pub fn from_byte(byte: u8) -> Result<Self, String> {
        // TODO: Implement mapping from byte to Opcode variant
    }
}

// TODO: Add necessary derive traits
pub struct UTXO {
    pub txid: Vec<u8>,
    pub vout: u32,
    pub value: u64,
}

pub fn consume_utxo(utxo: UTXO) -> UTXO {
    // TODO: Implement UTXO consumption logic (if any)
}
