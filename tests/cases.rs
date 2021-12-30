// Subset of test cases from https://github.com/cryptocoinjs/base-x/blob/master/test/fixtures.json
pub const TEST_CASES: &[(&[u8], &str)] = &[
    (&[], ""),
    (&[0xa8], "2g"),
    (&[137, 147, 212], "a3gV"),
    (&[138, 226, 28], "aPEr"),
    (&[190, 214, 100, 3], "3EFU7m"),
    (&[26, 44, 58, 220], "Rt5zm"),
    (&[148, 75, 238, 155, 50], "ABnLTmg"),
    (
        &[2, 218, 186, 98, 85, 3, 21, 23, 32, 91],
        "3SEo3LWLoPntC",
    ),
    (
        &[2, 234, 6, 153, 155, 153, 213, 38, 207, 203, 74],
        "EJDM8drfXA6uyA",
    ),
    (
        &[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
        "0000000000",
    ),
    (
        &[3, 35, 31, 241, 9, 99, 171, 195, 201, 208, 184],
        "FPBt6CHo3fovdL",
    ),
    (
        &[
            4, 118, 90, 245, 77, 141, 45, 171, 82, 225, 224, 233, 70, 91,
        ],
        "NKioeUVktgzXLJ1B3t",
    ),
    (
        &[0x00, 3,3,3,3,3,3,3,3, 3,3,3,3,3,3,3,3, 3,3,3,3,3,3,3,3, 3,3,3,3,3,3,3,3,],
        "0MzZaKLuYG3Mok8_uglyFlT8aQgzsMb49HIJbSdHp1F",
    ),
    (
        &[0x01, 3,3,3,3,3,3,3,3, 3,3,3,3,3,3,3,3, 3,3,3,3,3,3,3,3, 3,3,3,3,3,3,3,3,],
        "VN2ZamuBlvb8F_HsNNTFVnmijQgSCqbffb_l84dJehV",
    ),
    (
        &[0x01, 255,255,255,255,255,255,255,255, 255,255,255,255,255,255,255,255, 255,255,255,255,255,255,255,255, 255,255,255,255,255,255,255,255,],
        "_0800v5aRK4XuWHjxOPaW4d9H_zwgy1C1fPt4F03gLV",
    ),
    (
        &[0x02, 255,255,255,255,255,255,255,255, 255,255,255,255,255,255,255,255, 255,255,255,255,255,255,255,255, 255,255,255,255,255,255,255,255,],
        "1U0C01MdsezcJLmQbQ56sm6wjQ_zP1R1nX_7Jbs05W0l",
    ),
];

#[cfg(feature = "check")]
pub const CHECK_TEST_CASES: &[(&[u8], &str)] = &[
    (&[], "3QJmnh"),
    (&[0x31], "6bdbJ1U"),
    (&[0x39], "7VsrQCP"),
    (&[0x2d, 0x31], "PWEu9GGN"),
    (&[0x31, 0x31], "RVnPfpC2"),
    (
        &[0x31, 0x32, 0x33, 0x34, 0x35, 0x39, 0x38, 0x37, 0x36, 0x30],
        "K5zqBMZZTzUbAZQgrt4",
    ),
    (
        &[
            0x00, 0x9b, 0x41, 0x54, 0xbb, 0xf2, 0x03, 0xe4, 0x13, 0x0c, 0x4b, 0x86, 0x25, 0x93,
            0x18, 0xa4, 0x98, 0x75, 0xdd, 0x04, 0x56,
        ],
        "1F9v11cupBVMpz3CrVfCppv9Rw2xEtU1c6",
    ),
    (
        &[
            0x53, 0x25, 0xb1, 0xe2, 0x3b, 0x5b, 0x24, 0xf3, 0x47, 0xed, 0x19, 0xde, 0x61, 0x23,
            0x8a, 0xf1, 0x4b, 0xc4, 0x71, 0xca, 0xa1, 0xa7, 0x7a, 0xa5, 0x5d, 0xb2, 0xa7, 0xaf,
            0x7d, 0xaa, 0x93, 0xaa,
        ],
        "dctKSXBbv2My3TGGUgTFjkxu1A9JM3Sscd5FydY4dkxnfwA7q",
    ),
];
