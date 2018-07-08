#![feature(test)]

extern crate hex;

mod gf28;

// const INPUT_BLOCK_LENGTH: u8 = 128;
// const OUTPUT_BLOCK_LENGTH: u8 = 128;
// const STATE_LENGTH: u8 = 128;
// const KEY_LENGTH: u32 = 256;
const NB: usize = 4;
const NK: usize = 8;
const NR: usize = 14;

type Word = u32;
type State = [u32; NB as usize];
type Key = u32;
type KeySchedule = [Key; (NB * (NR + 1)) as usize];

// https://en.wikipedia.org/wiki/Rijndael_key_schedule#Rcon
static RCON:[u32; 14] = [0x8d, 0x01, 0x02, 0x04, 0x08, 0x10, 0x20, 0x40, 0x80, 0x1b, 0x36, 0x6c, 0xd8, 0xab];

// https://en.wikipedia.org/wiki/Rijndael_S-box
static SBOX: [u32; 256] = [
	0x63, 0x7C, 0x77, 0x7B, 0xF2, 0x6B, 0x6F, 0xC5, 0x30, 0x01, 0x67, 0x2B, 0xFE, 0xD7, 0xAB, 0x76,
    0xCA, 0x82, 0xC9, 0x7D, 0xFA, 0x59, 0x47, 0xF0, 0xAD, 0xD4, 0xA2, 0xAF, 0x9C, 0xA4, 0x72, 0xC0,
    0xB7, 0xFD, 0x93, 0x26, 0x36, 0x3F, 0xF7, 0xCC, 0x34, 0xA5, 0xE5, 0xF1, 0x71, 0xD8, 0x31, 0x15,
    0x04, 0xC7, 0x23, 0xC3, 0x18, 0x96, 0x05, 0x9A, 0x07, 0x12, 0x80, 0xE2, 0xEB, 0x27, 0xB2, 0x75,
    0x09, 0x83, 0x2C, 0x1A, 0x1B, 0x6E, 0x5A, 0xA0, 0x52, 0x3B, 0xD6, 0xB3, 0x29, 0xE3, 0x2F, 0x84,
    0x53, 0xD1, 0x00, 0xED, 0x20, 0xFC, 0xB1, 0x5B, 0x6A, 0xCB, 0xBE, 0x39, 0x4A, 0x4C, 0x58, 0xCF,
    0xD0, 0xEF, 0xAA, 0xFB, 0x43, 0x4D, 0x33, 0x85, 0x45, 0xF9, 0x02, 0x7F, 0x50, 0x3C, 0x9F, 0xA8,
    0x51, 0xA3, 0x40, 0x8F, 0x92, 0x9D, 0x38, 0xF5, 0xBC, 0xB6, 0xDA, 0x21, 0x10, 0xFF, 0xF3, 0xD2,
    0xCD, 0x0C, 0x13, 0xEC, 0x5F, 0x97, 0x44, 0x17, 0xC4, 0xA7, 0x7E, 0x3D, 0x64, 0x5D, 0x19, 0x73,
    0x60, 0x81, 0x4F, 0xDC, 0x22, 0x2A, 0x90, 0x88, 0x46, 0xEE, 0xB8, 0x14, 0xDE, 0x5E, 0x0B, 0xDB,
    0xE0, 0x32, 0x3A, 0x0A, 0x49, 0x06, 0x24, 0x5C, 0xC2, 0xD3, 0xAC, 0x62, 0x91, 0x95, 0xE4, 0x79,
    0xE7, 0xC8, 0x37, 0x6D, 0x8D, 0xD5, 0x4E, 0xA9, 0x6C, 0x56, 0xF4, 0xEA, 0x65, 0x7A, 0xAE, 0x08,
    0xBA, 0x78, 0x25, 0x2E, 0x1C, 0xA6, 0xB4, 0xC6, 0xE8, 0xDD, 0x74, 0x1F, 0x4B, 0xBD, 0x8B, 0x8A,
    0x70, 0x3E, 0xB5, 0x66, 0x48, 0x03, 0xF6, 0x0E, 0x61, 0x35, 0x57, 0xB9, 0x86, 0xC1, 0x1D, 0x9E,
    0xE1, 0xF8, 0x98, 0x11, 0x69, 0xD9, 0x8E, 0x94, 0x9B, 0x1E, 0x87, 0xE9, 0xCE, 0x55, 0x28, 0xDF,
    0x8C, 0xA1, 0x89, 0x0D, 0xBF, 0xE6, 0x42, 0x68, 0x41, 0x99, 0x2D, 0x0F, 0xB0, 0x54, 0xBB, 0x16
];

static GMUL2: [u8; 256] = [
	0x00,0x02,0x04,0x06,0x08,0x0a,0x0c,0x0e,0x10,0x12,0x14,0x16,0x18,0x1a,0x1c,0x1e,
	0x20,0x22,0x24,0x26,0x28,0x2a,0x2c,0x2e,0x30,0x32,0x34,0x36,0x38,0x3a,0x3c,0x3e,
	0x40,0x42,0x44,0x46,0x48,0x4a,0x4c,0x4e,0x50,0x52,0x54,0x56,0x58,0x5a,0x5c,0x5e,
	0x60,0x62,0x64,0x66,0x68,0x6a,0x6c,0x6e,0x70,0x72,0x74,0x76,0x78,0x7a,0x7c,0x7e,
	0x80,0x82,0x84,0x86,0x88,0x8a,0x8c,0x8e,0x90,0x92,0x94,0x96,0x98,0x9a,0x9c,0x9e,
	0xa0,0xa2,0xa4,0xa6,0xa8,0xaa,0xac,0xae,0xb0,0xb2,0xb4,0xb6,0xb8,0xba,0xbc,0xbe,
	0xc0,0xc2,0xc4,0xc6,0xc8,0xca,0xcc,0xce,0xd0,0xd2,0xd4,0xd6,0xd8,0xda,0xdc,0xde,
	0xe0,0xe2,0xe4,0xe6,0xe8,0xea,0xec,0xee,0xf0,0xf2,0xf4,0xf6,0xf8,0xfa,0xfc,0xfe,
	0x1b,0x19,0x1f,0x1d,0x13,0x11,0x17,0x15,0x0b,0x09,0x0f,0x0d,0x03,0x01,0x07,0x05,
	0x3b,0x39,0x3f,0x3d,0x33,0x31,0x37,0x35,0x2b,0x29,0x2f,0x2d,0x23,0x21,0x27,0x25,
	0x5b,0x59,0x5f,0x5d,0x53,0x51,0x57,0x55,0x4b,0x49,0x4f,0x4d,0x43,0x41,0x47,0x45,
	0x7b,0x79,0x7f,0x7d,0x73,0x71,0x77,0x75,0x6b,0x69,0x6f,0x6d,0x63,0x61,0x67,0x65,
	0x9b,0x99,0x9f,0x9d,0x93,0x91,0x97,0x95,0x8b,0x89,0x8f,0x8d,0x83,0x81,0x87,0x85,
	0xbb,0xb9,0xbf,0xbd,0xb3,0xb1,0xb7,0xb5,0xab,0xa9,0xaf,0xad,0xa3,0xa1,0xa7,0xa5,
	0xdb,0xd9,0xdf,0xdd,0xd3,0xd1,0xd7,0xd5,0xcb,0xc9,0xcf,0xcd,0xc3,0xc1,0xc7,0xc5,
	0xfb,0xf9,0xff,0xfd,0xf3,0xf1,0xf7,0xf5,0xeb,0xe9,0xef,0xed,0xe3,0xe1,0xe7,0xe5
];

static GMUL3: [u8; 256] = [
	0x00,0x03,0x06,0x05,0x0c,0x0f,0x0a,0x09,0x18,0x1b,0x1e,0x1d,0x14,0x17,0x12,0x11,
	0x30,0x33,0x36,0x35,0x3c,0x3f,0x3a,0x39,0x28,0x2b,0x2e,0x2d,0x24,0x27,0x22,0x21,
	0x60,0x63,0x66,0x65,0x6c,0x6f,0x6a,0x69,0x78,0x7b,0x7e,0x7d,0x74,0x77,0x72,0x71,
	0x50,0x53,0x56,0x55,0x5c,0x5f,0x5a,0x59,0x48,0x4b,0x4e,0x4d,0x44,0x47,0x42,0x41,
	0xc0,0xc3,0xc6,0xc5,0xcc,0xcf,0xca,0xc9,0xd8,0xdb,0xde,0xdd,0xd4,0xd7,0xd2,0xd1,
	0xf0,0xf3,0xf6,0xf5,0xfc,0xff,0xfa,0xf9,0xe8,0xeb,0xee,0xed,0xe4,0xe7,0xe2,0xe1,
	0xa0,0xa3,0xa6,0xa5,0xac,0xaf,0xaa,0xa9,0xb8,0xbb,0xbe,0xbd,0xb4,0xb7,0xb2,0xb1,
	0x90,0x93,0x96,0x95,0x9c,0x9f,0x9a,0x99,0x88,0x8b,0x8e,0x8d,0x84,0x87,0x82,0x81,
	0x9b,0x98,0x9d,0x9e,0x97,0x94,0x91,0x92,0x83,0x80,0x85,0x86,0x8f,0x8c,0x89,0x8a,
	0xab,0xa8,0xad,0xae,0xa7,0xa4,0xa1,0xa2,0xb3,0xb0,0xb5,0xb6,0xbf,0xbc,0xb9,0xba,
	0xfb,0xf8,0xfd,0xfe,0xf7,0xf4,0xf1,0xf2,0xe3,0xe0,0xe5,0xe6,0xef,0xec,0xe9,0xea,
	0xcb,0xc8,0xcd,0xce,0xc7,0xc4,0xc1,0xc2,0xd3,0xd0,0xd5,0xd6,0xdf,0xdc,0xd9,0xda,
	0x5b,0x58,0x5d,0x5e,0x57,0x54,0x51,0x52,0x43,0x40,0x45,0x46,0x4f,0x4c,0x49,0x4a,
	0x6b,0x68,0x6d,0x6e,0x67,0x64,0x61,0x62,0x73,0x70,0x75,0x76,0x7f,0x7c,0x79,0x7a,
	0x3b,0x38,0x3d,0x3e,0x37,0x34,0x31,0x32,0x23,0x20,0x25,0x26,0x2f,0x2c,0x29,0x2a,
	0x0b,0x08,0x0d,0x0e,0x07,0x04,0x01,0x02,0x13,0x10,0x15,0x16,0x1f,0x1c,0x19,0x1a
];

pub fn encrypt(input: &mut State, key: &str) {
	let tmp = hex::decode(key).unwrap();
	let mut key: [u8; 32] = [0; 32];
	key.copy_from_slice(&tmp[..]);
	let w: KeySchedule = key_expansion(key);
	cipher(input, w);
}

fn cipher(state: &mut State, w: KeySchedule) {
	add_round_key(state, &w[0..NB]);
	for round in 1..NR {
		sub_bytes(state);
		shift_rows(state);
		mix_columns(state);
		add_round_key(state, &w[round*NB..(round+1)*NB]);
	}
	sub_bytes(state);
	shift_rows(state);
	add_round_key(state, &w[NR*NB..(NR + 1)*NB]);
}

fn sub_bytes(state: &mut State) {
	for word in state {
		*word = sub_word(*word);
	}
}

fn shift_rows(state: &mut State) {
	for r in 1..4 {
		let mut row: u32 = 0;
		for column in 0..4 {
			row |= (extract_nth_byte(state[column], r) as u32) << (24 - column * 8);
		}
		row = shift_left(row, r);
		for c in 0..4 {
			state[c] = clear_nth_byte(state[c], r);
			state[c] = insert_nth_byte(state[c], extract_nth_byte(row, c) as u32, r);
		}
	}
}

fn mix_columns(state: &mut State) {
	for column in 0..4 {
		state[column] = mix_column(state[column]);
	}	
}

fn mix_column(c: Word) -> Word {
	// https://en.wikipedia.org/wiki/Rijndael_MixColumns
	let s0: u32 = (GMUL2[extract_nth_byte(c, 0) as usize] ^ 
					GMUL3[extract_nth_byte(c, 1) as usize] ^ 
					extract_nth_byte(c, 2) ^ 
					extract_nth_byte(c, 3)).into();
	let s1: u32 = (extract_nth_byte(c, 0) ^ 
					GMUL2[extract_nth_byte(c, 1) as usize] ^
					GMUL3[extract_nth_byte(c, 2) as usize] ^
					extract_nth_byte(c, 3)).into();
	let s2: u32 = (extract_nth_byte(c, 0) ^ 
					extract_nth_byte(c, 1) ^ 
					GMUL2[extract_nth_byte(c, 2) as usize] ^ 
					GMUL3[extract_nth_byte(c, 3) as usize]).into();
	let s3: u32 = (GMUL3[extract_nth_byte(c, 0) as usize] ^ 
					extract_nth_byte(c, 1) ^ 
					extract_nth_byte(c, 2) ^ 
					GMUL2[extract_nth_byte(c, 3) as usize]).into();
	(s0 << 24 | s1 << 16 | s2 << 8 | s3).into()
}

fn add_round_key(state: &mut State, w: &[Key]) {
	for c in 0..NB {
		state[c] ^= w[c];
	}
}

fn key_expansion(key: [u8; (4 * NK)]) -> KeySchedule {
	let mut w: KeySchedule = [0; 60];
	let mut temp: Word;
	for i in 0..NK {
		w[i] = 
			(key[(4 * i)] as u32) << 24 | 
			(key[(4 * i + 1)] as u32) << 16 |
     		(key[(4 * i + 2)] as u32) << 8 | 
     		(key[(4 * i + 3)] as u32);
	}
	for i in NK..NB * (NR + 1) {
		temp = w[i-1];
		if i % NK == 0 {
			temp = sub_word(rot_word(temp)) ^ (RCON[i/NK]<<24);
		} else if NK > 6 && i % NK == 4 {
			temp = sub_word(temp);
		};
		w[i] = w[i-NK] ^ temp;
	}
	w
}

fn sub_word(word: Word) -> Word {
	SBOX[(word >> 24) as usize] << 24 |
		SBOX[(word >> 16&0xff) as usize] << 16 |
		SBOX[(word >> 8&0xff) as usize] << 8 |
		SBOX[(word&0xff) as usize]
}

fn rot_word(word: Word) -> Word {
	word << 8 | word >> 24
}


fn extract_nth_byte(word: Word, n: usize) -> u8 {
	((word >> (24 - n*8)) & 0xff) as u8
}

fn clear_nth_byte(word: Word, n: usize) -> u32 {
	word & !(0xff << (24 - n*8))
}

fn insert_nth_byte(word: Word, i: u32, n: usize) -> u32 {
	word | ((i as u32) << (24 - n * 8))
}

fn shift_left(block: Word, count: usize) -> Word {
	block << (count * 8) | block >> (32 - count * 8)
}

#[cfg(test)]
mod tests {

	use super::*;
	extern crate itertools;
	use tests::itertools::Itertools;
	use std::u32;

	// Test vector from FIPS 197 A.3
	// http://www.csrc.nist.gov/publications/fips/fips197/fips-197.pdf
	static KEY_EXPANSION_KEY: [u8; 32] = [0x60, 0x3d, 0xeb, 0x10, 0x15, 0xca, 0x71, 0xbe, 0x2b, 0x73, 0xae, 0xf0, 0x85, 0x7d, 0x77, 0x81,
			0x1f, 0x35, 0x2c, 0x07, 0x3b, 0x61, 0x08, 0xd7, 0x2d, 0x98, 0x10, 0xa3, 0x09, 0x14, 0xdf, 0xf4];
	static KEY_EXPANSION_SCHED: KeySchedule = [
			0x603deb10, 0x15ca71be, 0x2b73aef0, 0x857d7781,
			0x1f352c07, 0x3b6108d7, 0x2d9810a3, 0x0914dff4,
			0x9ba35411, 0x8e6925af, 0xa51a8b5f, 0x2067fcde,
			0xa8b09c1a, 0x93d194cd, 0xbe49846e, 0xb75d5b9a,
			0xd59aecb8, 0x5bf3c917, 0xfee94248, 0xde8ebe96,
			0xb5a9328a, 0x2678a647, 0x98312229, 0x2f6c79b3,
			0x812c81ad, 0xdadf48ba, 0x24360af2, 0xfab8b464,
			0x98c5bfc9, 0xbebd198e, 0x268c3ba7, 0x09e04214,
			0x68007bac, 0xb2df3316, 0x96e939e4, 0x6c518d80,
			0xc814e204, 0x76a9fb8a, 0x5025c02d, 0x59c58239,
			0xde136967, 0x6ccc5a71, 0xfa256395, 0x9674ee15,
			0x5886ca5d, 0x2e2f31d7, 0x7e0af1fa, 0x27cf73c3,
			0x749c47ab, 0x18501dda, 0xe2757e4f, 0x7401905a,
			0xcafaaae3, 0xe4d59b34, 0x9adf6ace, 0xbd10190d,
			0xfe4890d1, 0xe6188d0b, 0x046df344, 0x706c631e,
			];
	#[test]
    fn key_expansion_test() {
    	let w = key_expansion(KEY_EXPANSION_KEY);
    	for i in 0..60 {
    		assert_eq!(w[i], KEY_EXPANSION_SCHED[i]);
    	}
    }

    #[test]
    fn shift_rows_test() {
        let mut state = [0x63c0ab20, 0xeb2f30cb, 0x9f93af2b, 0xa092c7a2];
        let expected = [0x632fafa2, 0xeb93c720, 0x9f92abcb, 0xa0c0302b];
        shift_rows(&mut state);
        assert_eq!(state, expected);
    }

    #[test]
    fn sub_word_test() {
        assert_eq!(sub_word(0x00041214), 0x63f2c9fa);
    }

    #[test]
    fn sub_bytes_test() {
    	// http://www.cs.siue.edu/~tgamage/S17/CS490/L/WK05.pdf
        let mut state = [0x00041214, 0x12041200, 0x0c001311, 0x08231919];
        sub_bytes(&mut state);
        assert_eq!(state, [0x63f2c9fa, 0xc9f2c963, 0xfe637d82, 0x3026d4d4]);
    }

    #[test]
    fn extract_nth_byte_test() {
        assert_eq!(extract_nth_byte(0x603deb10, 0), 0x60);
        assert_eq!(extract_nth_byte(0x603deb10, 1), 0x3d);
        assert_eq!(extract_nth_byte(0x603deb10, 2), 0xeb);
        assert_eq!(extract_nth_byte(0x603deb10, 3), 0x10);
    }

    #[test]
    fn clear_nth_byte_test() {
    	assert_eq!(clear_nth_byte(0x603deb10, 0), 0x003deb10);
        assert_eq!(clear_nth_byte(0x603deb10, 1), 0x6000eb10);
        assert_eq!(clear_nth_byte(0x603deb10, 2), 0x603d0010);
        assert_eq!(clear_nth_byte(0x603deb10, 3), 0x603deb00);
    }

    #[test]
    fn insert_nth_byte_test() {
        assert_eq!(insert_nth_byte(clear_nth_byte(0x603deb10, 0), 0xff, 0), 0xff3deb10);
        assert_eq!(insert_nth_byte(clear_nth_byte(0x603deb10, 1), 0xff, 1), 0x60ffeb10);
        assert_eq!(insert_nth_byte(clear_nth_byte(0x603deb10, 2), 0xff, 2), 0x603dff10);
        assert_eq!(insert_nth_byte(clear_nth_byte(0x603deb10, 3), 0xff, 3), 0x603debff);
    }

    #[test]
    fn shift_left_test() {
		assert_eq!(shift_left(0x603deb10, 1), 0x3deb1060);
		assert_eq!(shift_left(0x603deb10, 2), 0xeb10603d);
		assert_eq!(shift_left(0x603deb10, 3), 0x10603deb);
    }

    #[test]
    fn mix_column_test() {
    	// https://crypto.stackexchange.com/questions/2402/how-to-solve-mixcolumns
        assert_eq!(mix_column(0xd4bf5d30), 0x046681e5);
    }

    #[test]
    fn mix_columns_test() {
    	// https://en.wikipedia.org/wiki/Rijndael_MixColumns#Test_vectors_for_MixColumn();_not_for_InvMixColumn
		let mut original = [0xdb135345, 0xf20a225c, 0x01010101, 0xc6c6c6c6];
		let want = [0x8e4da1bc, 0x9fdc589d, 0x01010101, 0xc6c6c6c6];
		mix_columns(&mut original);
		assert_eq!(original, want);
    }

    #[test]
    fn add_round_key_test() {
    	let key = "000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f";
    	let tmp = hex::decode(key).unwrap();
		let mut key: [u8; 32] = [0; 32];
		key.copy_from_slice(&tmp[..]);
		let w: KeySchedule = key_expansion(key);
		// let mut state: State = hex_to_u32arr("00112233445566778899aabbccddeeff");
		// let mut state = i64::from_str_radix("00112233445566778899aabbccddeeff", 16);
		let plaintext = "00112233445566778899aabbccddeeff";
		let mut state = hex_to_u32arr(plaintext);

		let expected = "00102030405060708090a0b0c0d0e0f0";
		let want = hex_to_u32arr(expected);
		
		add_round_key(&mut state, &w[0..NB]);
		assert_eq!(state, want);
    }

    #[test]
    fn encrypt_test() {
        let key = "000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f";
        // let mut plaintext = "00112233445566778899aabbccddeeff".as_bytes();


        let plaintext = "00112233445566778899aabbccddeeff";
        // let mut plaintext = [0x00112233, 0x44556677, 0x8899aabb, 0xccddeeff];
        let mut plaintext = hex_to_u32arr(plaintext);
        encrypt(&mut plaintext, key);
        println!("{:02x}", plaintext.iter().format(""));
    }

    const START: usize = 0;   
    const S_BOX: usize = 1;
    const S_ROW: usize = 2;
    const M_COL: usize = 3;
    const K_SCH: usize = 4;

    #[test]
    fn debug() {
    	let key = "000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f";
    	let tmp = hex::decode(key).unwrap();
		let mut key: [u8; 32] = [0; 32];
		key.copy_from_slice(&tmp[..]);
		let w: KeySchedule = key_expansion(key);
		
		let plaintext = "00112233445566778899aabbccddeeff";
		let mut state = hex_to_u32arr(plaintext);
		
		let test_table = init_test_table();
		let round0 = [
			hex_to_u32arr("00112233445566778899aabbccddeeff"),
			hex_to_u32arr("000102030405060708090a0b0c0d0e0f"),
		];
		let round14 = [
			hex_to_u32arr("627bceb9999d5aaac945ecf423f56da5"),
			hex_to_u32arr("aa218b56ee5ebeacdd6ecebf26e63c06"),
			hex_to_u32arr("aa5ece06ee6e3c56dde68bac2621bebf"),
			hex_to_u32arr("24fc79ccbf0979e9371ac23c6d68de36"),
			hex_to_u32arr("8ea2b7ca516745bfeafc49904b496089"),
		];
		// println!("{:?}", &w[..]);
		assert_eq!(state, round0[0]);
		assert_eq!(&w[0..NB], round0[1]);
        add_round_key(&mut state, &w[0..NB]);
		for round in 1..NR {
			println!("round {:?}", round);
			assert_eq!(state, test_table[round][START]);
			sub_bytes(&mut state);
			assert_eq!(state, test_table[round][S_BOX]);
			shift_rows(&mut state);
			assert_eq!(state, test_table[round][S_ROW]);
			mix_columns(&mut state);
			assert_eq!(state, test_table[round][M_COL]);
			println!("{} to {}", round*NB, (round+1)*NB);
			assert_eq!(&w[round*NB..(round+1)*NB], test_table[round][K_SCH]);
			// AddRoundKey(state, w[round*Nb, (round+1)*Nb-1])
			add_round_key(&mut state, &w[round*NB..(round+1)*NB]);
		}
		assert_eq!(state, round14[0]);
		sub_bytes(&mut state);
		assert_eq!(state, round14[1]);
		shift_rows(&mut state);
		assert_eq!(state, round14[2]);
		assert_eq!(&w[NR*NB..(NR + 1)*NB], round14[3]);
		add_round_key(&mut state, &w[NR*NB..(NR + 1)*NB]);
		assert_eq!(state, round14[4]);
    }

    fn hex_to_u32arr(hx: &str) -> [u32; 4] {
    	let mut state: State = [0; NB];
    	for i in 0..4 {
			state[i] = u32::from_str_radix(&hx[i*8..i*8+8], 16).unwrap();
		}
		state
    }

    fn init_test_table() -> [[[u32; 4]; 5]; 14] {
    	[	
    		[
    			[0,0,0,0], [0,0,0,0] ,[0,0,0,0] ,[0,0,0,0] ,[0,0,0,0]
    		],
			[
				hex_to_u32arr("00102030405060708090a0b0c0d0e0f0"),
				hex_to_u32arr("63cab7040953d051cd60e0e7ba70e18c"),
				hex_to_u32arr("6353e08c0960e104cd70b751bacad0e7"),
				hex_to_u32arr("5f72641557f5bc92f7be3b291db9f91a"),
				hex_to_u32arr("101112131415161718191a1b1c1d1e1f"),
			],
			[
				hex_to_u32arr("4f63760643e0aa85efa7213201a4e705"),
				hex_to_u32arr("84fb386f1ae1ac97df5cfd237c49946b"),
				hex_to_u32arr("84e1fd6b1a5c946fdf4938977cfbac23"),
				hex_to_u32arr("bd2a395d2b6ac438d192443e615da195"),
				hex_to_u32arr("a573c29fa176c498a97fce93a572c09c"),
			],
			[
				hex_to_u32arr("1859fbc28a1c00a078ed8aadc42f6109"),
				hex_to_u32arr("adcb0f257e9c63e0bc557e951c15ef01"),
				hex_to_u32arr("ad9c7e017e55ef25bc150fe01ccb6395"),
				hex_to_u32arr("810dce0cc9db8172b3678c1e88a1b5bd"),
				hex_to_u32arr("1651a8cd0244beda1a5da4c10640bade"),
			],
			[
				hex_to_u32arr("975c66c1cb9f3fa8a93a28df8ee10f63"),
				hex_to_u32arr("884a33781fdb75c2d380349e19f876fb"),
				hex_to_u32arr("88db34fb1f807678d3f833c2194a759e"),
				hex_to_u32arr("b2822d81abe6fb275faf103a078c0033"),
				hex_to_u32arr("ae87dff00ff11b68a68ed5fb03fc1567"),
			],
			[
				hex_to_u32arr("1c05f271a417e04ff921c5c104701554"),
				hex_to_u32arr("9c6b89a349f0e18499fda678f2515920"),
				hex_to_u32arr("9cf0a62049fd59a399518984f26be178"),
				hex_to_u32arr("aeb65ba974e0f822d73f567bdb64c877"),
				hex_to_u32arr("6de1f1486fa54f9275f8eb5373b8518d"),
			],
			[
				hex_to_u32arr("c357aae11b45b7b0a2c7bd28a8dc99fa"),
				hex_to_u32arr("2e5bacf8af6ea9e73ac67a34c286ee2d"),
				hex_to_u32arr("2e6e7a2dafc6eef83a86ace7c25ba934"),
				hex_to_u32arr("b951c33c02e9bd29ae25cdb1efa08cc7"),
				hex_to_u32arr("c656827fc9a799176f294cec6cd5598b"),
			],
			[
				hex_to_u32arr("7f074143cb4e243ec10c815d8375d54c"),
				hex_to_u32arr("d2c5831a1f2f36b278fe0c4cec9d0329"),
				hex_to_u32arr("d22f0c291ffe031a789d83b2ecc5364c"),
				hex_to_u32arr("ebb19e1c3ee7c9e87d7535e9ed6b9144"),
				hex_to_u32arr("3de23a75524775e727bf9eb45407cf39"),
			],
			[
				hex_to_u32arr("d653a4696ca0bc0f5acaab5db96c5e7d"),
				hex_to_u32arr("f6ed49f950e06576be74624c565058ff"),
				hex_to_u32arr("f6e062ff507458f9be50497656ed654c"),
				hex_to_u32arr("5174c8669da98435a8b3e62ca974a5ea"),
				hex_to_u32arr("0bdc905fc27b0948ad5245a4c1871c2f"),
			],
			[
				hex_to_u32arr("5aa858395fd28d7d05e1a38868f3b9c5"),
				hex_to_u32arr("bec26a12cfb55dff6bf80ac4450d56a6"),
				hex_to_u32arr("beb50aa6cff856126b0d6aff45c25dc4"),
				hex_to_u32arr("0f77ee31d2ccadc05430a83f4ef96ac3"),
				hex_to_u32arr("45f5a66017b2d387300d4d33640a820a"),
			],
			[
				hex_to_u32arr("4a824851c57e7e47643de50c2af3e8c9"),
				hex_to_u32arr("d61352d1a6f3f3a04327d9fee50d9bdd"),
				hex_to_u32arr("d6f3d9dda6279bd1430d52a0e513f3fe"),
				hex_to_u32arr("bd86f0ea748fc4f4630f11c1e9331233"),
				hex_to_u32arr("7ccff71cbeb4fe5413e6bbf0d261a7df"),
			],
			[
				hex_to_u32arr("c14907f6ca3b3aa070e9aa313b52b5ec"),
				hex_to_u32arr("783bc54274e280e0511eacc7e200d5ce"),
				hex_to_u32arr("78e2acce741ed5425100c5e0e23b80c7"),
				hex_to_u32arr("af8690415d6e1dd387e5fbedd5c89013"),
				hex_to_u32arr("f01afafee7a82979d7a5644ab3afe640"),
			],
			[
				hex_to_u32arr("5f9c6abfbac634aa50409fa766677653"),
				hex_to_u32arr("cfde0208f4b418ac5309db5c338538ed"),
				hex_to_u32arr("cfb4dbedf4093808538502ac33de185c"),
				hex_to_u32arr("7427fae4d8a695269ce83d315be0392b"),
				hex_to_u32arr("2541fe719bf500258813bbd55a721c0a"),
			],
			[
				hex_to_u32arr("516604954353950314fb86e401922521"),
				hex_to_u32arr("d133f22a1aed2a7bfa0f44697c4f3ffd"),
				hex_to_u32arr("d1ed44fd1a0f3f2afa4ff27b7c332a69"),
				hex_to_u32arr("2c21a820306f154ab712c75eee0da04f"),
				hex_to_u32arr("4e5a6699a9f24fe07e572baacdf8cdea"),
			]
		]
    }
}
