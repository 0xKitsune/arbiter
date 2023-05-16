pub use uniswap_v2_liquidity_math_library::*;
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
pub mod uniswap_v2_liquidity_math_library {
    #[rustfmt::skip]
    const __ABI: &str = "[]";
    ///The parsed JSON ABI of the contract.
    pub static UNISWAPV2LIQUIDITYMATHLIBRARY_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(|| {
        ::ethers::core::utils::__serde_json::from_str(__ABI).expect("ABI is always valid")
    });
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = &[
        96,
        175,
        96,
        35,
        96,
        11,
        130,
        130,
        130,
        57,
        128,
        81,
        96,
        0,
        26,
        96,
        115,
        20,
        96,
        22,
        87,
        254,
        91,
        48,
        96,
        0,
        82,
        96,
        115,
        129,
        83,
        130,
        129,
        243,
        254,
        115,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        48,
        20,
        96,
        128,
        96,
        64,
        82,
        98,
        70,
        27,
        205,
        96,
        229,
        27,
        96,
        0,
        144,
        129,
        82,
        96,
        32,
        96,
        4,
        82,
        96,
        53,
        96,
        36,
        82,
        127,
        67,
        111,
        110,
        116,
        114,
        97,
        99,
        116,
        32,
        100,
        111,
        101,
        115,
        32,
        110,
        111,
        116,
        32,
        104,
        97,
        118,
        101,
        32,
        102,
        97,
        108,
        108,
        98,
        97,
        99,
        107,
        32,
        96,
        68,
        144,
        129,
        82,
        116,
        110,
        111,
        114,
        32,
        114,
        101,
        99,
        101,
        105,
        118,
        101,
        32,
        102,
        117,
        110,
        99,
        116,
        105,
        111,
        110,
        115,
        96,
        88,
        27,
        96,
        100,
        82,
        144,
        96,
        132,
        144,
        253,
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
        138,
        178,
        13,
        71,
        11,
        216,
        4,
        202,
        164,
        24,
        248,
        116,
        194,
        213,
        160,
        69,
        35,
        98,
        119,
        3,
        201,
        107,
        15,
        151,
        17,
        77,
        242,
        196,
        194,
        74,
        86,
        224,
        100,
        115,
        111,
        108,
        99,
        67,
        0,
        6,
        6,
        0,
        51,
    ];
    ///The bytecode of the contract.
    pub static UNISWAPV2LIQUIDITYMATHLIBRARY_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = &[
        115,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        48,
        20,
        96,
        128,
        96,
        64,
        82,
        98,
        70,
        27,
        205,
        96,
        229,
        27,
        96,
        0,
        144,
        129,
        82,
        96,
        32,
        96,
        4,
        82,
        96,
        53,
        96,
        36,
        82,
        127,
        67,
        111,
        110,
        116,
        114,
        97,
        99,
        116,
        32,
        100,
        111,
        101,
        115,
        32,
        110,
        111,
        116,
        32,
        104,
        97,
        118,
        101,
        32,
        102,
        97,
        108,
        108,
        98,
        97,
        99,
        107,
        32,
        96,
        68,
        144,
        129,
        82,
        116,
        110,
        111,
        114,
        32,
        114,
        101,
        99,
        101,
        105,
        118,
        101,
        32,
        102,
        117,
        110,
        99,
        116,
        105,
        111,
        110,
        115,
        96,
        88,
        27,
        96,
        100,
        82,
        144,
        96,
        132,
        144,
        253,
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
        138,
        178,
        13,
        71,
        11,
        216,
        4,
        202,
        164,
        24,
        248,
        116,
        194,
        213,
        160,
        69,
        35,
        98,
        119,
        3,
        201,
        107,
        15,
        151,
        17,
        77,
        242,
        196,
        194,
        74,
        86,
        224,
        100,
        115,
        111,
        108,
        99,
        67,
        0,
        6,
        6,
        0,
        51,
    ];
    ///The deployed bytecode of the contract.
    pub static UNISWAPV2LIQUIDITYMATHLIBRARY_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct UniswapV2LiquidityMathLibrary<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for UniswapV2LiquidityMathLibrary<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for UniswapV2LiquidityMathLibrary<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for UniswapV2LiquidityMathLibrary<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for UniswapV2LiquidityMathLibrary<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(UniswapV2LiquidityMathLibrary))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> UniswapV2LiquidityMathLibrary<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                UNISWAPV2LIQUIDITYMATHLIBRARY_ABI.clone(),
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
                UNISWAPV2LIQUIDITYMATHLIBRARY_ABI.clone(),
                UNISWAPV2LIQUIDITYMATHLIBRARY_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
        for UniswapV2LiquidityMathLibrary<M>
    {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
}
