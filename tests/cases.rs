// Subset of test cases from https://github.com/cryptocoinjs/base-x/blob/master/test/fixtures.json
pub const TEST_CASES: &[(&[u8], &str)] = &[
    (&[], ""),
    (&[169], "2g"),
    (&[141, 100, 210], "a3gV"),
    (&[142, 178, 220], "aPEr"),
    (&[190, 214, 100, 4], "3EFU7m"),
    (&[26, 48, 11, 219], "Rt5zm"),
    (&[148, 76, 222, 250, 115], "ABnLTmg"),
    (
        &[2, 218, 186, 153, 223, 226, 25, 85, 81, 91],
        "3SEo3LWLoPntC",
    ),
    (
        &[2, 234, 6, 153, 156, 127, 43, 24, 45, 57, 75],
        "EJDM8drfXA6uyA",
    ),
    (
        &[0x01, 255,255,255,255,255,255,255,255, 255,255,255,255,255,255,255,255, 255,255,255,255,255,255,255,255, 255,255,255,255,255,255,255,255,],
        "z0800u5_RK4XtWHiwOP_W4c9Hzyvfx1C1ePs4F03fLV",
    ),
    (
        &[0x02, 255,255,255,255,255,255,255,255, 255,255,255,255,255,255,255,255, 255,255,255,255,255,255,255,255, 255,255,255,255,255,255,255,255,],
        "1U0C01McrdybJLlQaQ56rl6viQzyP1R1mXz7Jar05W0k",
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
