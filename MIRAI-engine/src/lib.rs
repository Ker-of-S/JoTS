use wasm_bindgen::prelude::*;
use num_bigint::BigUint;
use num_traits::{Zero, ToPrimitive};

const CHARSET: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789 \\{}[]^_%$#&~-.,:;!?()+=*/<>|@";
const PAGE_LENGTH: usize = 800;

fn calculate_offset(node: u32, cluster: u32, frag: u32) -> u32 {
    node * 100000 + cluster * 1000 + frag
}

fn get_avalanche_shift(offset: u32, index: usize, base_len: usize) -> usize {
    let mut state = offset.wrapping_add((index as u32).wrapping_mul(0x9E3779B9));
    state ^= state >> 16;
    state = state.wrapping_mul(0x85ebca6b);
    state ^= state >> 13;
    state = state.wrapping_mul(0xc2b2ae35);
    state ^= state >> 16;
    (state as usize) % base_len 
}

#[wasm_bindgen]
pub fn explore_coordinates(sector_hex: &str, node: u32, cluster: u32, frag: u32) -> String {
    let hex_str = if sector_hex.to_lowercase().starts_with("0x") { &sector_hex[2..] } else { sector_hex };
    let sector_mass = match BigUint::parse_bytes(hex_str.as_bytes(), 16) {
        Some(num) => num,
        None => return "ERROR: INVALID_SECTOR".to_string(),
    };

    let chars: Vec<char> = CHARSET.chars().collect();
    let base_len = chars.len();
    let base = BigUint::from(base_len);
    let mut current_number = sector_mass;
    let mut base_text = String::new();
    let zero = BigUint::zero();

    while current_number > zero {
        let remainder = (&current_number % &base).to_u32().unwrap() as usize;
        base_text.insert(0, chars[remainder]);
        current_number /= &base;
    }
    while base_text.chars().count() < PAGE_LENGTH {
        base_text.insert(0, chars[0]);
    }

    let offset = calculate_offset(node, cluster, frag);
    let mut final_text = String::new();
    for (i, c) in base_text.chars().enumerate() {
        let char_idx = chars.iter().position(|&x| x == c).unwrap_or(0);
        let shift = get_avalanche_shift(offset, i, base_len);
        let new_idx = (char_idx + shift) % base_len; 
        final_text.push(chars[new_idx]);
    }
    final_text
}

#[wasm_bindgen]
pub fn search_database(query: &str) -> String {
    let chars: Vec<char> = CHARSET.chars().collect();
    let base_len = chars.len();
    let clean_query: String = query.chars().filter(|c| chars.contains(c)).collect();
    
    if clean_query.is_empty() { return r#"{"error": true}"#.to_string(); }

    let mut full_page = String::new();
    let mut seed = (clean_query.len() as f64).sin().abs() * 100.0;
    let insert_pos = (seed % (PAGE_LENGTH - clean_query.chars().count()) as f64) as usize;

    for i in 0..PAGE_LENGTH {
        if i == insert_pos {
            full_page.push_str(&clean_query);
        } else if i < insert_pos || i >= insert_pos + clean_query.chars().count() {
            let pseudo_rand = ((i as f64 + seed).cos().abs() * base_len as f64) as usize;
            full_page.push(chars[pseudo_rand % base_len]);
        }
    }

    let mock_node = (clean_query.len() as u32 % 4) + 1;
    let mock_cluster = (clean_query.len() as u32 % 5) + 1;
    let mock_frag = (clean_query.len() as u32 % 32) + 1;
    let offset = calculate_offset(mock_node, mock_cluster, mock_frag);

    let mut base_text = String::new();
    for (i, c) in full_page.chars().enumerate() {
        let char_idx = chars.iter().position(|&x| x == c).unwrap_or(0);
        let shift = get_avalanche_shift(offset, i, base_len);
        let base_idx = (char_idx + base_len - shift) % base_len; 
        base_text.push(chars[base_idx]);
    }

    let base_biguint = BigUint::from(base_len);
    let mut quantum_number = BigUint::zero();
    for c in base_text.chars() {
        let char_index = chars.iter().position(|&x| x == c).unwrap_or(0);
        quantum_number = (quantum_number * &base_biguint) + BigUint::from(char_index);
    }

    format!(r#"{{"error": false, "sector": "0x{:X}", "node": {}, "cluster": {}, "frag": {}, "content": "{}"}}"#, 
        quantum_number, mock_node, mock_cluster, mock_frag, full_page.replace("\\", "\\\\").replace("\"", "\\\""))
}