pub use writer::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types
)]
pub mod writer {
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[{\"internalType\":\"string\",\"name\":\"test_string\",\"type\":\"string\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"WasWritten\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"test_string\",\"type\":\"string\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"echoString\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]}]";
    ///The parsed JSON ABI of the contract.
    pub static WRITER_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(|| {
            ::ethers::core::utils::__serde_json::from_str(__ABI).expect("ABI is always valid")
        });
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = &[
        96,
        128,
        96,
        64,
        82,
        52,
        128,
        21,
        97,
        0,
        16,
        87,
        96,
        0,
        128,
        253,
        91,
        80,
        97,
        3,
        197,
        128,
        97,
        0,
        32,
        96,
        0,
        57,
        96,
        0,
        243,
        254,
        96,
        128,
        96,
        64,
        82,
        52,
        128,
        21,
        97,
        0,
        16,
        87,
        96,
        0,
        128,
        253,
        91,
        80,
        96,
        4,
        54,
        16,
        97,
        0,
        43,
        87,
        96,
        0,
        53,
        96,
        224,
        28,
        128,
        99,
        13,
        126,
        47,
        206,
        20,
        97,
        0,
        48,
        87,
        91,
        96,
        0,
        128,
        253,
        91,
        97,
        0,
        67,
        97,
        0,
        62,
        54,
        96,
        4,
        97,
        1,
        71,
        86,
        91,
        97,
        0,
        89,
        86,
        91,
        96,
        64,
        81,
        97,
        0,
        80,
        145,
        144,
        97,
        1,
        248,
        86,
        91,
        96,
        64,
        81,
        128,
        145,
        3,
        144,
        243,
        91,
        96,
        96,
        96,
        0,
        97,
        0,
        103,
        131,
        130,
        97,
        2,
        207,
        86,
        91,
        80,
        127,
        150,
        111,
        235,
        127,
        22,
        48,
        252,
        188,
        148,
        154,
        121,
        101,
        185,
        139,
        203,
        203,
        152,
        35,
        112,
        159,
        116,
        217,
        236,
        10,
        84,
        130,
        176,
        195,
        48,
        148,
        139,
        148,
        130,
        96,
        64,
        81,
        97,
        0,
        151,
        145,
        144,
        97,
        1,
        248,
        86,
        91,
        96,
        64,
        81,
        128,
        145,
        3,
        144,
        161,
        96,
        0,
        128,
        84,
        97,
        0,
        172,
        144,
        97,
        2,
        70,
        86,
        91,
        128,
        96,
        31,
        1,
        96,
        32,
        128,
        145,
        4,
        2,
        96,
        32,
        1,
        96,
        64,
        81,
        144,
        129,
        1,
        96,
        64,
        82,
        128,
        146,
        145,
        144,
        129,
        129,
        82,
        96,
        32,
        1,
        130,
        128,
        84,
        97,
        0,
        216,
        144,
        97,
        2,
        70,
        86,
        91,
        128,
        21,
        97,
        1,
        37,
        87,
        128,
        96,
        31,
        16,
        97,
        0,
        250,
        87,
        97,
        1,
        0,
        128,
        131,
        84,
        4,
        2,
        131,
        82,
        145,
        96,
        32,
        1,
        145,
        97,
        1,
        37,
        86,
        91,
        130,
        1,
        145,
        144,
        96,
        0,
        82,
        96,
        32,
        96,
        0,
        32,
        144,
        91,
        129,
        84,
        129,
        82,
        144,
        96,
        1,
        1,
        144,
        96,
        32,
        1,
        128,
        131,
        17,
        97,
        1,
        8,
        87,
        130,
        144,
        3,
        96,
        31,
        22,
        130,
        1,
        145,
        91,
        80,
        80,
        80,
        80,
        80,
        144,
        80,
        145,
        144,
        80,
        86,
        91,
        99,
        78,
        72,
        123,
        113,
        96,
        224,
        27,
        96,
        0,
        82,
        96,
        65,
        96,
        4,
        82,
        96,
        36,
        96,
        0,
        253,
        91,
        96,
        0,
        96,
        32,
        130,
        132,
        3,
        18,
        21,
        97,
        1,
        89,
        87,
        96,
        0,
        128,
        253,
        91,
        129,
        53,
        103,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        128,
        130,
        17,
        21,
        97,
        1,
        113,
        87,
        96,
        0,
        128,
        253,
        91,
        129,
        132,
        1,
        145,
        80,
        132,
        96,
        31,
        131,
        1,
        18,
        97,
        1,
        133,
        87,
        96,
        0,
        128,
        253,
        91,
        129,
        53,
        129,
        129,
        17,
        21,
        97,
        1,
        151,
        87,
        97,
        1,
        151,
        97,
        1,
        49,
        86,
        91,
        96,
        64,
        81,
        96,
        31,
        130,
        1,
        96,
        31,
        25,
        144,
        129,
        22,
        96,
        63,
        1,
        22,
        129,
        1,
        144,
        131,
        130,
        17,
        129,
        131,
        16,
        23,
        21,
        97,
        1,
        191,
        87,
        97,
        1,
        191,
        97,
        1,
        49,
        86,
        91,
        129,
        96,
        64,
        82,
        130,
        129,
        82,
        135,
        96,
        32,
        132,
        135,
        1,
        1,
        17,
        21,
        97,
        1,
        216,
        87,
        96,
        0,
        128,
        253,
        91,
        130,
        96,
        32,
        134,
        1,
        96,
        32,
        131,
        1,
        55,
        96,
        0,
        146,
        129,
        1,
        96,
        32,
        1,
        146,
        144,
        146,
        82,
        80,
        149,
        148,
        80,
        80,
        80,
        80,
        80,
        86,
        91,
        96,
        0,
        96,
        32,
        128,
        131,
        82,
        131,
        81,
        128,
        130,
        133,
        1,
        82,
        96,
        0,
        91,
        129,
        129,
        16,
        21,
        97,
        2,
        37,
        87,
        133,
        129,
        1,
        131,
        1,
        81,
        133,
        130,
        1,
        96,
        64,
        1,
        82,
        130,
        1,
        97,
        2,
        9,
        86,
        91,
        80,
        96,
        0,
        96,
        64,
        130,
        134,
        1,
        1,
        82,
        96,
        64,
        96,
        31,
        25,
        96,
        31,
        131,
        1,
        22,
        133,
        1,
        1,
        146,
        80,
        80,
        80,
        146,
        145,
        80,
        80,
        86,
        91,
        96,
        1,
        129,
        129,
        28,
        144,
        130,
        22,
        128,
        97,
        2,
        90,
        87,
        96,
        127,
        130,
        22,
        145,
        80,
        91,
        96,
        32,
        130,
        16,
        129,
        3,
        97,
        2,
        122,
        87,
        99,
        78,
        72,
        123,
        113,
        96,
        224,
        27,
        96,
        0,
        82,
        96,
        34,
        96,
        4,
        82,
        96,
        36,
        96,
        0,
        253,
        91,
        80,
        145,
        144,
        80,
        86,
        91,
        96,
        31,
        130,
        17,
        21,
        97,
        2,
        202,
        87,
        96,
        0,
        129,
        129,
        82,
        96,
        32,
        129,
        32,
        96,
        31,
        133,
        1,
        96,
        5,
        28,
        129,
        1,
        96,
        32,
        134,
        16,
        21,
        97,
        2,
        167,
        87,
        80,
        128,
        91,
        96,
        31,
        133,
        1,
        96,
        5,
        28,
        130,
        1,
        145,
        80,
        91,
        129,
        129,
        16,
        21,
        97,
        2,
        198,
        87,
        130,
        129,
        85,
        96,
        1,
        1,
        97,
        2,
        179,
        86,
        91,
        80,
        80,
        80,
        91,
        80,
        80,
        80,
        86,
        91,
        129,
        81,
        103,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        129,
        17,
        21,
        97,
        2,
        233,
        87,
        97,
        2,
        233,
        97,
        1,
        49,
        86,
        91,
        97,
        2,
        253,
        129,
        97,
        2,
        247,
        132,
        84,
        97,
        2,
        70,
        86,
        91,
        132,
        97,
        2,
        128,
        86,
        91,
        96,
        32,
        128,
        96,
        31,
        131,
        17,
        96,
        1,
        129,
        20,
        97,
        3,
        50,
        87,
        96,
        0,
        132,
        21,
        97,
        3,
        26,
        87,
        80,
        133,
        131,
        1,
        81,
        91,
        96,
        0,
        25,
        96,
        3,
        134,
        144,
        27,
        28,
        25,
        22,
        96,
        1,
        133,
        144,
        27,
        23,
        133,
        85,
        97,
        2,
        198,
        86,
        91,
        96,
        0,
        133,
        129,
        82,
        96,
        32,
        129,
        32,
        96,
        31,
        25,
        134,
        22,
        145,
        91,
        130,
        129,
        16,
        21,
        97,
        3,
        97,
        87,
        136,
        134,
        1,
        81,
        130,
        85,
        148,
        132,
        1,
        148,
        96,
        1,
        144,
        145,
        1,
        144,
        132,
        1,
        97,
        3,
        66,
        86,
        91,
        80,
        133,
        130,
        16,
        21,
        97,
        3,
        127,
        87,
        135,
        133,
        1,
        81,
        96,
        0,
        25,
        96,
        3,
        136,
        144,
        27,
        96,
        248,
        22,
        28,
        25,
        22,
        129,
        85,
        91,
        80,
        80,
        80,
        80,
        80,
        96,
        1,
        144,
        129,
        27,
        1,
        144,
        85,
        80,
        86,
        254,
        162,
        100,
        105,
        112,
        102,
        115,
        88,
        34,
        18,
        32,
        233,
        163,
        220,
        111,
        153,
        164,
        138,
        152,
        206,
        40,
        141,
        223,
        7,
        195,
        2,
        242,
        132,
        29,
        171,
        157,
        186,
        23,
        155,
        30,
        137,
        253,
        26,
        180,
        59,
        16,
        142,
        16,
        100,
        115,
        111,
        108,
        99,
        67,
        0,
        8,
        19,
        0,
        51,
    ];
    ///The bytecode of the contract.
    pub static WRITER_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = &[
        96,
        128,
        96,
        64,
        82,
        52,
        128,
        21,
        97,
        0,
        16,
        87,
        96,
        0,
        128,
        253,
        91,
        80,
        96,
        4,
        54,
        16,
        97,
        0,
        43,
        87,
        96,
        0,
        53,
        96,
        224,
        28,
        128,
        99,
        13,
        126,
        47,
        206,
        20,
        97,
        0,
        48,
        87,
        91,
        96,
        0,
        128,
        253,
        91,
        97,
        0,
        67,
        97,
        0,
        62,
        54,
        96,
        4,
        97,
        1,
        71,
        86,
        91,
        97,
        0,
        89,
        86,
        91,
        96,
        64,
        81,
        97,
        0,
        80,
        145,
        144,
        97,
        1,
        248,
        86,
        91,
        96,
        64,
        81,
        128,
        145,
        3,
        144,
        243,
        91,
        96,
        96,
        96,
        0,
        97,
        0,
        103,
        131,
        130,
        97,
        2,
        207,
        86,
        91,
        80,
        127,
        150,
        111,
        235,
        127,
        22,
        48,
        252,
        188,
        148,
        154,
        121,
        101,
        185,
        139,
        203,
        203,
        152,
        35,
        112,
        159,
        116,
        217,
        236,
        10,
        84,
        130,
        176,
        195,
        48,
        148,
        139,
        148,
        130,
        96,
        64,
        81,
        97,
        0,
        151,
        145,
        144,
        97,
        1,
        248,
        86,
        91,
        96,
        64,
        81,
        128,
        145,
        3,
        144,
        161,
        96,
        0,
        128,
        84,
        97,
        0,
        172,
        144,
        97,
        2,
        70,
        86,
        91,
        128,
        96,
        31,
        1,
        96,
        32,
        128,
        145,
        4,
        2,
        96,
        32,
        1,
        96,
        64,
        81,
        144,
        129,
        1,
        96,
        64,
        82,
        128,
        146,
        145,
        144,
        129,
        129,
        82,
        96,
        32,
        1,
        130,
        128,
        84,
        97,
        0,
        216,
        144,
        97,
        2,
        70,
        86,
        91,
        128,
        21,
        97,
        1,
        37,
        87,
        128,
        96,
        31,
        16,
        97,
        0,
        250,
        87,
        97,
        1,
        0,
        128,
        131,
        84,
        4,
        2,
        131,
        82,
        145,
        96,
        32,
        1,
        145,
        97,
        1,
        37,
        86,
        91,
        130,
        1,
        145,
        144,
        96,
        0,
        82,
        96,
        32,
        96,
        0,
        32,
        144,
        91,
        129,
        84,
        129,
        82,
        144,
        96,
        1,
        1,
        144,
        96,
        32,
        1,
        128,
        131,
        17,
        97,
        1,
        8,
        87,
        130,
        144,
        3,
        96,
        31,
        22,
        130,
        1,
        145,
        91,
        80,
        80,
        80,
        80,
        80,
        144,
        80,
        145,
        144,
        80,
        86,
        91,
        99,
        78,
        72,
        123,
        113,
        96,
        224,
        27,
        96,
        0,
        82,
        96,
        65,
        96,
        4,
        82,
        96,
        36,
        96,
        0,
        253,
        91,
        96,
        0,
        96,
        32,
        130,
        132,
        3,
        18,
        21,
        97,
        1,
        89,
        87,
        96,
        0,
        128,
        253,
        91,
        129,
        53,
        103,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        128,
        130,
        17,
        21,
        97,
        1,
        113,
        87,
        96,
        0,
        128,
        253,
        91,
        129,
        132,
        1,
        145,
        80,
        132,
        96,
        31,
        131,
        1,
        18,
        97,
        1,
        133,
        87,
        96,
        0,
        128,
        253,
        91,
        129,
        53,
        129,
        129,
        17,
        21,
        97,
        1,
        151,
        87,
        97,
        1,
        151,
        97,
        1,
        49,
        86,
        91,
        96,
        64,
        81,
        96,
        31,
        130,
        1,
        96,
        31,
        25,
        144,
        129,
        22,
        96,
        63,
        1,
        22,
        129,
        1,
        144,
        131,
        130,
        17,
        129,
        131,
        16,
        23,
        21,
        97,
        1,
        191,
        87,
        97,
        1,
        191,
        97,
        1,
        49,
        86,
        91,
        129,
        96,
        64,
        82,
        130,
        129,
        82,
        135,
        96,
        32,
        132,
        135,
        1,
        1,
        17,
        21,
        97,
        1,
        216,
        87,
        96,
        0,
        128,
        253,
        91,
        130,
        96,
        32,
        134,
        1,
        96,
        32,
        131,
        1,
        55,
        96,
        0,
        146,
        129,
        1,
        96,
        32,
        1,
        146,
        144,
        146,
        82,
        80,
        149,
        148,
        80,
        80,
        80,
        80,
        80,
        86,
        91,
        96,
        0,
        96,
        32,
        128,
        131,
        82,
        131,
        81,
        128,
        130,
        133,
        1,
        82,
        96,
        0,
        91,
        129,
        129,
        16,
        21,
        97,
        2,
        37,
        87,
        133,
        129,
        1,
        131,
        1,
        81,
        133,
        130,
        1,
        96,
        64,
        1,
        82,
        130,
        1,
        97,
        2,
        9,
        86,
        91,
        80,
        96,
        0,
        96,
        64,
        130,
        134,
        1,
        1,
        82,
        96,
        64,
        96,
        31,
        25,
        96,
        31,
        131,
        1,
        22,
        133,
        1,
        1,
        146,
        80,
        80,
        80,
        146,
        145,
        80,
        80,
        86,
        91,
        96,
        1,
        129,
        129,
        28,
        144,
        130,
        22,
        128,
        97,
        2,
        90,
        87,
        96,
        127,
        130,
        22,
        145,
        80,
        91,
        96,
        32,
        130,
        16,
        129,
        3,
        97,
        2,
        122,
        87,
        99,
        78,
        72,
        123,
        113,
        96,
        224,
        27,
        96,
        0,
        82,
        96,
        34,
        96,
        4,
        82,
        96,
        36,
        96,
        0,
        253,
        91,
        80,
        145,
        144,
        80,
        86,
        91,
        96,
        31,
        130,
        17,
        21,
        97,
        2,
        202,
        87,
        96,
        0,
        129,
        129,
        82,
        96,
        32,
        129,
        32,
        96,
        31,
        133,
        1,
        96,
        5,
        28,
        129,
        1,
        96,
        32,
        134,
        16,
        21,
        97,
        2,
        167,
        87,
        80,
        128,
        91,
        96,
        31,
        133,
        1,
        96,
        5,
        28,
        130,
        1,
        145,
        80,
        91,
        129,
        129,
        16,
        21,
        97,
        2,
        198,
        87,
        130,
        129,
        85,
        96,
        1,
        1,
        97,
        2,
        179,
        86,
        91,
        80,
        80,
        80,
        91,
        80,
        80,
        80,
        86,
        91,
        129,
        81,
        103,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        129,
        17,
        21,
        97,
        2,
        233,
        87,
        97,
        2,
        233,
        97,
        1,
        49,
        86,
        91,
        97,
        2,
        253,
        129,
        97,
        2,
        247,
        132,
        84,
        97,
        2,
        70,
        86,
        91,
        132,
        97,
        2,
        128,
        86,
        91,
        96,
        32,
        128,
        96,
        31,
        131,
        17,
        96,
        1,
        129,
        20,
        97,
        3,
        50,
        87,
        96,
        0,
        132,
        21,
        97,
        3,
        26,
        87,
        80,
        133,
        131,
        1,
        81,
        91,
        96,
        0,
        25,
        96,
        3,
        134,
        144,
        27,
        28,
        25,
        22,
        96,
        1,
        133,
        144,
        27,
        23,
        133,
        85,
        97,
        2,
        198,
        86,
        91,
        96,
        0,
        133,
        129,
        82,
        96,
        32,
        129,
        32,
        96,
        31,
        25,
        134,
        22,
        145,
        91,
        130,
        129,
        16,
        21,
        97,
        3,
        97,
        87,
        136,
        134,
        1,
        81,
        130,
        85,
        148,
        132,
        1,
        148,
        96,
        1,
        144,
        145,
        1,
        144,
        132,
        1,
        97,
        3,
        66,
        86,
        91,
        80,
        133,
        130,
        16,
        21,
        97,
        3,
        127,
        87,
        135,
        133,
        1,
        81,
        96,
        0,
        25,
        96,
        3,
        136,
        144,
        27,
        96,
        248,
        22,
        28,
        25,
        22,
        129,
        85,
        91,
        80,
        80,
        80,
        80,
        80,
        96,
        1,
        144,
        129,
        27,
        1,
        144,
        85,
        80,
        86,
        254,
        162,
        100,
        105,
        112,
        102,
        115,
        88,
        34,
        18,
        32,
        233,
        163,
        220,
        111,
        153,
        164,
        138,
        152,
        206,
        40,
        141,
        223,
        7,
        195,
        2,
        242,
        132,
        29,
        171,
        157,
        186,
        23,
        155,
        30,
        137,
        253,
        26,
        180,
        59,
        16,
        142,
        16,
        100,
        115,
        111,
        108,
        99,
        67,
        0,
        8,
        19,
        0,
        51,
    ];
    ///The deployed bytecode of the contract.
    pub static WRITER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct Writer<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for Writer<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for Writer<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for Writer<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for Writer<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(Writer))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> Writer<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                WRITER_ABI.clone(),
                client,
            ))
        }
        /// Constructs the general purpose `Deployer` instance based on the provided constructor arguments and sends it.
        /// Returns a new instance of a deployer that returns an instance of this contract after sending the transaction
        ///
        /// Notes:
        /// - If there are no constructor arguments, you should pass `()` as the argument.
        /// - The default poll duration is 7 seconds.
        /// - The default number of confirmations is 1 block.
        ///
        ///
        /// # Example
        ///
        /// Generate contract bindings with `abigen!` and deploy a new contract instance.
        ///
        /// *Note*: this requires a `bytecode` and `abi` object in the `greeter.json` artifact.
        ///
        /// ```ignore
        /// # async fn deploy<M: ethers::providers::Middleware>(client: ::std::sync::Arc<M>) {
        ///     abigen!(Greeter, "../greeter.json");
        ///
        ///    let greeter_contract = Greeter::deploy(client, "Hello world!".to_string()).unwrap().send().await.unwrap();
        ///    let msg = greeter_contract.greet().call().await.unwrap();
        /// # }
        /// ```
        pub fn deploy<T: ::ethers::core::abi::Tokenize>(
            client: ::std::sync::Arc<M>,
            constructor_args: T,
        ) -> ::core::result::Result<
            ::ethers::contract::builders::ContractDeployer<M, Self>,
            ::ethers::contract::ContractError<M>,
        > {
            let factory = ::ethers::contract::ContractFactory::new(
                WRITER_ABI.clone(),
                WRITER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `echoString` (0x0d7e2fce) function
        pub fn echo_string(
            &self,
            test_string: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([13, 126, 47, 206], test_string)
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `WasWritten` event
        pub fn was_written_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, WasWrittenFilter> {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, WasWrittenFilter> {
            self.0
                .event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>> for Writer<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(name = "WasWritten", abi = "WasWritten(string)")]
    pub struct WasWrittenFilter {
        pub test_string: ::std::string::String,
    }
    ///Container type for all input parameters for the `echoString` function with signature `echoString(string)` and selector `0x0d7e2fce`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "echoString", abi = "echoString(string)")]
    pub struct EchoStringCall {
        pub test_string: ::std::string::String,
    }
    ///Container type for all return fields from the `echoString` function with signature `echoString(string)` and selector `0x0d7e2fce`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct EchoStringReturn(pub ::std::string::String);
}