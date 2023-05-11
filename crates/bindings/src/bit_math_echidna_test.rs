pub use bit_math_echidna_test::*;
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
pub mod bit_math_echidna_test {
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"input\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"leastSignificantBitInvariant\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"input\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"mostSignificantBitInvariant\",\"outputs\":[]}]";
    ///The parsed JSON ABI of the contract.
    pub static BITMATHECHIDNATEST_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
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
        2,
        178,
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
        54,
        87,
        96,
        0,
        53,
        96,
        224,
        28,
        128,
        99,
        45,
        113,
        30,
        12,
        20,
        97,
        0,
        59,
        87,
        128,
        99,
        249,
        74,
        201,
        14,
        20,
        97,
        0,
        90,
        87,
        91,
        96,
        0,
        128,
        253,
        91,
        97,
        0,
        88,
        96,
        4,
        128,
        54,
        3,
        96,
        32,
        129,
        16,
        21,
        97,
        0,
        81,
        87,
        96,
        0,
        128,
        253,
        91,
        80,
        53,
        97,
        0,
        119,
        86,
        91,
        0,
        91,
        97,
        0,
        88,
        96,
        4,
        128,
        54,
        3,
        96,
        32,
        129,
        16,
        21,
        97,
        0,
        112,
        87,
        96,
        0,
        128,
        253,
        91,
        80,
        53,
        97,
        0,
        171,
        86,
        91,
        96,
        0,
        97,
        0,
        130,
        130,
        97,
        0,
        232,
        86,
        91,
        144,
        80,
        96,
        255,
        129,
        22,
        96,
        2,
        10,
        130,
        22,
        97,
        0,
        147,
        87,
        254,
        91,
        96,
        0,
        25,
        96,
        255,
        130,
        22,
        96,
        2,
        10,
        1,
        130,
        22,
        21,
        97,
        0,
        167,
        87,
        254,
        91,
        80,
        80,
        86,
        91,
        96,
        0,
        97,
        0,
        182,
        130,
        97,
        1,
        220,
        86,
        91,
        144,
        80,
        128,
        96,
        255,
        22,
        96,
        2,
        10,
        130,
        16,
        21,
        97,
        0,
        200,
        87,
        254,
        91,
        128,
        96,
        255,
        22,
        96,
        255,
        20,
        128,
        97,
        0,
        226,
        87,
        80,
        128,
        96,
        1,
        1,
        96,
        255,
        22,
        96,
        2,
        10,
        130,
        16,
        91,
        97,
        0,
        167,
        87,
        254,
        91,
        96,
        0,
        128,
        130,
        17,
        97,
        0,
        246,
        87,
        96,
        0,
        128,
        253,
        91,
        80,
        96,
        255,
        111,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        130,
        22,
        21,
        97,
        1,
        26,
        87,
        96,
        127,
        25,
        1,
        97,
        1,
        34,
        86,
        91,
        96,
        128,
        130,
        144,
        28,
        145,
        80,
        91,
        103,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        130,
        22,
        21,
        97,
        1,
        59,
        87,
        96,
        63,
        25,
        1,
        97,
        1,
        67,
        86,
        91,
        96,
        64,
        130,
        144,
        28,
        145,
        80,
        91,
        99,
        255,
        255,
        255,
        255,
        130,
        22,
        21,
        97,
        1,
        88,
        87,
        96,
        31,
        25,
        1,
        97,
        1,
        96,
        86,
        91,
        96,
        32,
        130,
        144,
        28,
        145,
        80,
        91,
        97,
        255,
        255,
        130,
        22,
        21,
        97,
        1,
        115,
        87,
        96,
        15,
        25,
        1,
        97,
        1,
        123,
        86,
        91,
        96,
        16,
        130,
        144,
        28,
        145,
        80,
        91,
        96,
        255,
        130,
        22,
        21,
        97,
        1,
        141,
        87,
        96,
        7,
        25,
        1,
        97,
        1,
        149,
        86,
        91,
        96,
        8,
        130,
        144,
        28,
        145,
        80,
        91,
        96,
        15,
        130,
        22,
        21,
        97,
        1,
        167,
        87,
        96,
        3,
        25,
        1,
        97,
        1,
        175,
        86,
        91,
        96,
        4,
        130,
        144,
        28,
        145,
        80,
        91,
        96,
        3,
        130,
        22,
        21,
        97,
        1,
        193,
        87,
        96,
        1,
        25,
        1,
        97,
        1,
        201,
        86,
        91,
        96,
        2,
        130,
        144,
        28,
        145,
        80,
        91,
        96,
        1,
        130,
        22,
        21,
        97,
        1,
        215,
        87,
        96,
        0,
        25,
        1,
        91,
        145,
        144,
        80,
        86,
        91,
        96,
        0,
        128,
        130,
        17,
        97,
        1,
        234,
        87,
        96,
        0,
        128,
        253,
        91,
        96,
        1,
        96,
        128,
        27,
        130,
        16,
        97,
        1,
        253,
        87,
        96,
        128,
        145,
        130,
        28,
        145,
        1,
        91,
        104,
        1,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        130,
        16,
        97,
        2,
        21,
        87,
        96,
        64,
        145,
        130,
        28,
        145,
        1,
        91,
        100,
        1,
        0,
        0,
        0,
        0,
        130,
        16,
        97,
        2,
        41,
        87,
        96,
        32,
        145,
        130,
        28,
        145,
        1,
        91,
        98,
        1,
        0,
        0,
        130,
        16,
        97,
        2,
        59,
        87,
        96,
        16,
        145,
        130,
        28,
        145,
        1,
        91,
        97,
        1,
        0,
        130,
        16,
        97,
        2,
        76,
        87,
        96,
        8,
        145,
        130,
        28,
        145,
        1,
        91,
        96,
        16,
        130,
        16,
        97,
        2,
        92,
        87,
        96,
        4,
        145,
        130,
        28,
        145,
        1,
        91,
        96,
        4,
        130,
        16,
        97,
        2,
        108,
        87,
        96,
        2,
        145,
        130,
        28,
        145,
        1,
        91,
        96,
        2,
        130,
        16,
        97,
        1,
        215,
        87,
        96,
        1,
        1,
        145,
        144,
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
        108,
        236,
        138,
        240,
        222,
        146,
        214,
        74,
        206,
        76,
        30,
        82,
        32,
        184,
        112,
        120,
        239,
        155,
        177,
        188,
        149,
        55,
        141,
        149,
        82,
        232,
        21,
        62,
        24,
        107,
        16,
        239,
        100,
        115,
        111,
        108,
        99,
        67,
        0,
        7,
        6,
        0,
        51,
    ];
    ///The bytecode of the contract.
    pub static BITMATHECHIDNATEST_BYTECODE: ::ethers::core::types::Bytes =
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
        54,
        87,
        96,
        0,
        53,
        96,
        224,
        28,
        128,
        99,
        45,
        113,
        30,
        12,
        20,
        97,
        0,
        59,
        87,
        128,
        99,
        249,
        74,
        201,
        14,
        20,
        97,
        0,
        90,
        87,
        91,
        96,
        0,
        128,
        253,
        91,
        97,
        0,
        88,
        96,
        4,
        128,
        54,
        3,
        96,
        32,
        129,
        16,
        21,
        97,
        0,
        81,
        87,
        96,
        0,
        128,
        253,
        91,
        80,
        53,
        97,
        0,
        119,
        86,
        91,
        0,
        91,
        97,
        0,
        88,
        96,
        4,
        128,
        54,
        3,
        96,
        32,
        129,
        16,
        21,
        97,
        0,
        112,
        87,
        96,
        0,
        128,
        253,
        91,
        80,
        53,
        97,
        0,
        171,
        86,
        91,
        96,
        0,
        97,
        0,
        130,
        130,
        97,
        0,
        232,
        86,
        91,
        144,
        80,
        96,
        255,
        129,
        22,
        96,
        2,
        10,
        130,
        22,
        97,
        0,
        147,
        87,
        254,
        91,
        96,
        0,
        25,
        96,
        255,
        130,
        22,
        96,
        2,
        10,
        1,
        130,
        22,
        21,
        97,
        0,
        167,
        87,
        254,
        91,
        80,
        80,
        86,
        91,
        96,
        0,
        97,
        0,
        182,
        130,
        97,
        1,
        220,
        86,
        91,
        144,
        80,
        128,
        96,
        255,
        22,
        96,
        2,
        10,
        130,
        16,
        21,
        97,
        0,
        200,
        87,
        254,
        91,
        128,
        96,
        255,
        22,
        96,
        255,
        20,
        128,
        97,
        0,
        226,
        87,
        80,
        128,
        96,
        1,
        1,
        96,
        255,
        22,
        96,
        2,
        10,
        130,
        16,
        91,
        97,
        0,
        167,
        87,
        254,
        91,
        96,
        0,
        128,
        130,
        17,
        97,
        0,
        246,
        87,
        96,
        0,
        128,
        253,
        91,
        80,
        96,
        255,
        111,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        130,
        22,
        21,
        97,
        1,
        26,
        87,
        96,
        127,
        25,
        1,
        97,
        1,
        34,
        86,
        91,
        96,
        128,
        130,
        144,
        28,
        145,
        80,
        91,
        103,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        130,
        22,
        21,
        97,
        1,
        59,
        87,
        96,
        63,
        25,
        1,
        97,
        1,
        67,
        86,
        91,
        96,
        64,
        130,
        144,
        28,
        145,
        80,
        91,
        99,
        255,
        255,
        255,
        255,
        130,
        22,
        21,
        97,
        1,
        88,
        87,
        96,
        31,
        25,
        1,
        97,
        1,
        96,
        86,
        91,
        96,
        32,
        130,
        144,
        28,
        145,
        80,
        91,
        97,
        255,
        255,
        130,
        22,
        21,
        97,
        1,
        115,
        87,
        96,
        15,
        25,
        1,
        97,
        1,
        123,
        86,
        91,
        96,
        16,
        130,
        144,
        28,
        145,
        80,
        91,
        96,
        255,
        130,
        22,
        21,
        97,
        1,
        141,
        87,
        96,
        7,
        25,
        1,
        97,
        1,
        149,
        86,
        91,
        96,
        8,
        130,
        144,
        28,
        145,
        80,
        91,
        96,
        15,
        130,
        22,
        21,
        97,
        1,
        167,
        87,
        96,
        3,
        25,
        1,
        97,
        1,
        175,
        86,
        91,
        96,
        4,
        130,
        144,
        28,
        145,
        80,
        91,
        96,
        3,
        130,
        22,
        21,
        97,
        1,
        193,
        87,
        96,
        1,
        25,
        1,
        97,
        1,
        201,
        86,
        91,
        96,
        2,
        130,
        144,
        28,
        145,
        80,
        91,
        96,
        1,
        130,
        22,
        21,
        97,
        1,
        215,
        87,
        96,
        0,
        25,
        1,
        91,
        145,
        144,
        80,
        86,
        91,
        96,
        0,
        128,
        130,
        17,
        97,
        1,
        234,
        87,
        96,
        0,
        128,
        253,
        91,
        96,
        1,
        96,
        128,
        27,
        130,
        16,
        97,
        1,
        253,
        87,
        96,
        128,
        145,
        130,
        28,
        145,
        1,
        91,
        104,
        1,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        130,
        16,
        97,
        2,
        21,
        87,
        96,
        64,
        145,
        130,
        28,
        145,
        1,
        91,
        100,
        1,
        0,
        0,
        0,
        0,
        130,
        16,
        97,
        2,
        41,
        87,
        96,
        32,
        145,
        130,
        28,
        145,
        1,
        91,
        98,
        1,
        0,
        0,
        130,
        16,
        97,
        2,
        59,
        87,
        96,
        16,
        145,
        130,
        28,
        145,
        1,
        91,
        97,
        1,
        0,
        130,
        16,
        97,
        2,
        76,
        87,
        96,
        8,
        145,
        130,
        28,
        145,
        1,
        91,
        96,
        16,
        130,
        16,
        97,
        2,
        92,
        87,
        96,
        4,
        145,
        130,
        28,
        145,
        1,
        91,
        96,
        4,
        130,
        16,
        97,
        2,
        108,
        87,
        96,
        2,
        145,
        130,
        28,
        145,
        1,
        91,
        96,
        2,
        130,
        16,
        97,
        1,
        215,
        87,
        96,
        1,
        1,
        145,
        144,
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
        108,
        236,
        138,
        240,
        222,
        146,
        214,
        74,
        206,
        76,
        30,
        82,
        32,
        184,
        112,
        120,
        239,
        155,
        177,
        188,
        149,
        55,
        141,
        149,
        82,
        232,
        21,
        62,
        24,
        107,
        16,
        239,
        100,
        115,
        111,
        108,
        99,
        67,
        0,
        7,
        6,
        0,
        51,
    ];
    ///The deployed bytecode of the contract.
    pub static BITMATHECHIDNATEST_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct BitMathEchidnaTest<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for BitMathEchidnaTest<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for BitMathEchidnaTest<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for BitMathEchidnaTest<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for BitMathEchidnaTest<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(BitMathEchidnaTest))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> BitMathEchidnaTest<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                BITMATHECHIDNATEST_ABI.clone(),
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
                BITMATHECHIDNATEST_ABI.clone(),
                BITMATHECHIDNATEST_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `leastSignificantBitInvariant` (0x2d711e0c) function
        pub fn least_significant_bit_invariant(
            &self,
            input: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([45, 113, 30, 12], input)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `mostSignificantBitInvariant` (0xf94ac90e) function
        pub fn most_significant_bit_invariant(
            &self,
            input: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([249, 74, 201, 14], input)
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
        for BitMathEchidnaTest<M>
    {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `leastSignificantBitInvariant` function with signature `leastSignificantBitInvariant(uint256)` and selector `0x2d711e0c`
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
    #[ethcall(
        name = "leastSignificantBitInvariant",
        abi = "leastSignificantBitInvariant(uint256)"
    )]
    pub struct LeastSignificantBitInvariantCall {
        pub input: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `mostSignificantBitInvariant` function with signature `mostSignificantBitInvariant(uint256)` and selector `0xf94ac90e`
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
    #[ethcall(
        name = "mostSignificantBitInvariant",
        abi = "mostSignificantBitInvariant(uint256)"
    )]
    pub struct MostSignificantBitInvariantCall {
        pub input: ::ethers::core::types::U256,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum BitMathEchidnaTestCalls {
        LeastSignificantBitInvariant(LeastSignificantBitInvariantCall),
        MostSignificantBitInvariant(MostSignificantBitInvariantCall),
    }
    impl ::ethers::core::abi::AbiDecode for BitMathEchidnaTestCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) =
                <LeastSignificantBitInvariantCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::LeastSignificantBitInvariant(decoded));
            }
            if let Ok(decoded) =
                <MostSignificantBitInvariantCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::MostSignificantBitInvariant(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for BitMathEchidnaTestCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::LeastSignificantBitInvariant(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MostSignificantBitInvariant(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for BitMathEchidnaTestCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::LeastSignificantBitInvariant(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::MostSignificantBitInvariant(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<LeastSignificantBitInvariantCall> for BitMathEchidnaTestCalls {
        fn from(value: LeastSignificantBitInvariantCall) -> Self {
            Self::LeastSignificantBitInvariant(value)
        }
    }
    impl ::core::convert::From<MostSignificantBitInvariantCall> for BitMathEchidnaTestCalls {
        fn from(value: MostSignificantBitInvariantCall) -> Self {
            Self::MostSignificantBitInvariant(value)
        }
    }
}
