
fn hex_to_u32arr(hx: &str) -> [u32; 4] {
	let tmp = hex::decode(hx).unwrap();
	let mut dec: [u32; 4] = [0; 4];
	for i in 0..4 {
		let i1 = i;
		let i = i * 4;
		dec[i1] = ((tmp[i] as u32) << 24 | (tmp[i + 1] as u32) << 16 | (tmp[i + 2] as u32) << 8 | tmp[i + 3] as u32).into();
	}
	dec
}


// static TEST_TABLE = [
// 	[
// 		hex_to_u32arr("00102030405060708090a0b0c0d0e0f0"),
// 		hex_to_u32arr("63cab7040953d051cd60e0e7ba70e18c"),
// 		hex_to_u32arr("6353e08c0960e104cd70b751bacad0e7"),
// 		hex_to_u32arr("5f72641557f5bc92f7be3b291db9f91a"),
// 		hex_to_u32arr("101112131415161718191a1b1c1d1e1f")
// 	],
// 	[
// 		hex_to_u32arr("00102030405060708090a0b0c0d0e0f0"),
// 		hex_to_u32arr("63cab7040953d051cd60e0e7ba70e18c"),
// 		hex_to_u32arr("6353e08c0960e104cd70b751bacad0e7"),
// 		hex_to_u32arr("5f72641557f5bc92f7be3b291db9f91a"),
// 		hex_to_u32arr("101112131415161718191a1b1c1d1e1f")
// 	],
// 	[
// 		hex_to_u32arr("00102030405060708090a0b0c0d0e0f0"),
// 		hex_to_u32arr("63cab7040953d051cd60e0e7ba70e18c"),
// 		hex_to_u32arr("6353e08c0960e104cd70b751bacad0e7"),
// 		hex_to_u32arr("5f72641557f5bc92f7be3b291db9f91a"),
// 		hex_to_u32arr("101112131415161718191a1b1c1d1e1f")
// 	],
// 	[
// 		hex_to_u32arr("00102030405060708090a0b0c0d0e0f0"),
// 		hex_to_u32arr("63cab7040953d051cd60e0e7ba70e18c"),
// 		hex_to_u32arr("6353e08c0960e104cd70b751bacad0e7"),
// 		hex_to_u32arr("5f72641557f5bc92f7be3b291db9f91a"),
// 		hex_to_u32arr("101112131415161718191a1b1c1d1e1f")
// 	],
// 	[
// 		hex_to_u32arr("00102030405060708090a0b0c0d0e0f0"),
// 		hex_to_u32arr("63cab7040953d051cd60e0e7ba70e18c"),
// 		hex_to_u32arr("6353e08c0960e104cd70b751bacad0e7"),
// 		hex_to_u32arr("5f72641557f5bc92f7be3b291db9f91a"),
// 		hex_to_u32arr("101112131415161718191a1b1c1d1e1f")
// 	],
// 	[
// 		hex_to_u32arr("00102030405060708090a0b0c0d0e0f0"),
// 		hex_to_u32arr("63cab7040953d051cd60e0e7ba70e18c"),
// 		hex_to_u32arr("6353e08c0960e104cd70b751bacad0e7"),
// 		hex_to_u32arr("5f72641557f5bc92f7be3b291db9f91a"),
// 		hex_to_u32arr("101112131415161718191a1b1c1d1e1f")
// 	],
// 	[
// 		hex_to_u32arr("00102030405060708090a0b0c0d0e0f0"),
// 		hex_to_u32arr("63cab7040953d051cd60e0e7ba70e18c"),
// 		hex_to_u32arr("6353e08c0960e104cd70b751bacad0e7"),
// 		hex_to_u32arr("5f72641557f5bc92f7be3b291db9f91a"),
// 		hex_to_u32arr("101112131415161718191a1b1c1d1e1f")
// 	],
// 	[
// 		hex_to_u32arr("00102030405060708090a0b0c0d0e0f0"),
// 		hex_to_u32arr("63cab7040953d051cd60e0e7ba70e18c"),
// 		hex_to_u32arr("6353e08c0960e104cd70b751bacad0e7"),
// 		hex_to_u32arr("5f72641557f5bc92f7be3b291db9f91a"),
// 		hex_to_u32arr("101112131415161718191a1b1c1d1e1f")
// 	],
// 	[
// 		hex_to_u32arr("00102030405060708090a0b0c0d0e0f0"),
// 		hex_to_u32arr("63cab7040953d051cd60e0e7ba70e18c"),
// 		hex_to_u32arr("6353e08c0960e104cd70b751bacad0e7"),
// 		hex_to_u32arr("5f72641557f5bc92f7be3b291db9f91a"),
// 		hex_to_u32arr("101112131415161718191a1b1c1d1e1f")
// 	],
// 	[
// 		hex_to_u32arr("00102030405060708090a0b0c0d0e0f0"),
// 		hex_to_u32arr("63cab7040953d051cd60e0e7ba70e18c"),
// 		hex_to_u32arr("6353e08c0960e104cd70b751bacad0e7"),
// 		hex_to_u32arr("5f72641557f5bc92f7be3b291db9f91a"),
// 		hex_to_u32arr("101112131415161718191a1b1c1d1e1f")
// 	],
// 	[
// 		hex_to_u32arr("00102030405060708090a0b0c0d0e0f0"),
// 		hex_to_u32arr("63cab7040953d051cd60e0e7ba70e18c"),
// 		hex_to_u32arr("6353e08c0960e104cd70b751bacad0e7"),
// 		hex_to_u32arr("5f72641557f5bc92f7be3b291db9f91a"),
// 		hex_to_u32arr("101112131415161718191a1b1c1d1e1f")
// 	],
// 	[
// 		hex_to_u32arr("00102030405060708090a0b0c0d0e0f0"),
// 		hex_to_u32arr("63cab7040953d051cd60e0e7ba70e18c"),
// 		hex_to_u32arr("6353e08c0960e104cd70b751bacad0e7"),
// 		hex_to_u32arr("5f72641557f5bc92f7be3b291db9f91a"),
// 		hex_to_u32arr("101112131415161718191a1b1c1d1e1f")
// 	],
// 	// [],
// 	// [],
// 	// [],
// 	// [],
// 	// [],
// 	// [],
// 	// [],
// 	// [],
// 	// [],
// 	// [],
// 	// []
// ];

// round[ 0].input    00112233445566778899aabbccddeeff
// round[ 0].k_sch    000102030405060708090a0b0c0d0e0f
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
// round[14].start    627bceb9999d5aaac945ecf423f56da5
// round[14].s_box    aa218b56ee5ebeacdd6ecebf26e63c06
// round[14].s_row    aa5ece06ee6e3c56dde68bac2621bebf
// round[14].k_sch    24fc79ccbf0979e9371ac23c6d68de36
// round[14].output   8ea2b7ca516745bfeafc49904b496089


// round[ 0].input    00112233445566778899aabbccddeeff
// round[ 0].k_sch    000102030405060708090a0b0c0d0e0f

round1 = [
	hex_to_u32arr("00112233445566778899aabbccddeeff"),
	hex_to_u32arr("000102030405060708090a0b0c0d0e0f"),
]

round14 = [
	hex_to_u32arr("627bceb9999d5aaac945ecf423f56da5"),
	hex_to_u32arr("aa218b56ee5ebeacdd6ecebf26e63c06"),
	hex_to_u32arr("aa5ece06ee6e3c56dde68bac2621bebf"),
	hex_to_u32arr("24fc79ccbf0979e9371ac23c6d68de36"),
	hex_to_u32arr("8ea2b7ca516745bfeafc49904b496089"),
]


round\[[0-9]+\]\.start\s+(.*)
round\[[0-9]+\]\.s_box\s+(.*)
round\[[0-9]+\]\.s_row\s+(.*)
round\[[0-9]+\]\.m_col\s+(.*)
round\[[0-9]+\]\.k_sch\s+(.*)

[
	hex_to_u32arr("$1"),
	hex_to_u32arr("$2"),
	hex_to_u32arr("$3"),
	hex_to_u32arr("$4"),
	hex_to_u32arr("$5"),
],