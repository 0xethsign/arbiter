pub use low_gas_safe_math_echidna_test::*;
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
pub mod low_gas_safe_math_echidna_test {
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"x\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"y\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"checkAdd\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"int256\",\"name\":\"x\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"y\",\"type\":\"int256\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"checkAddi\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"x\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"y\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"checkMul\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"x\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"y\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"checkSub\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"int256\",\"name\":\"x\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"y\",\"type\":\"int256\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"checkSubi\",\"outputs\":[]}]";
    ///The parsed JSON ABI of the contract.
    pub static LOWGASSAFEMATHECHIDNATEST_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
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
        181,
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
        87,
        87,
        96,
        0,
        53,
        96,
        224,
        28,
        128,
        99,
        15,
        147,
        80,
        0,
        20,
        97,
        0,
        92,
        87,
        128,
        99,
        15,
        228,
        140,
        92,
        20,
        97,
        0,
        129,
        87,
        128,
        99,
        109,
        136,
        111,
        174,
        20,
        97,
        0,
        164,
        87,
        128,
        99,
        146,
        57,
        231,
        119,
        20,
        97,
        0,
        199,
        87,
        128,
        99,
        189,
        120,
        178,
        13,
        20,
        97,
        0,
        234,
        87,
        91,
        96,
        0,
        128,
        253,
        91,
        97,
        0,
        127,
        96,
        4,
        128,
        54,
        3,
        96,
        64,
        129,
        16,
        21,
        97,
        0,
        114,
        87,
        96,
        0,
        128,
        253,
        91,
        80,
        128,
        53,
        144,
        96,
        32,
        1,
        53,
        97,
        1,
        13,
        86,
        91,
        0,
        91,
        97,
        0,
        127,
        96,
        4,
        128,
        54,
        3,
        96,
        64,
        129,
        16,
        21,
        97,
        0,
        151,
        87,
        96,
        0,
        128,
        253,
        91,
        80,
        128,
        53,
        144,
        96,
        32,
        1,
        53,
        97,
        1,
        65,
        86,
        91,
        97,
        0,
        127,
        96,
        4,
        128,
        54,
        3,
        96,
        64,
        129,
        16,
        21,
        97,
        0,
        186,
        87,
        96,
        0,
        128,
        253,
        91,
        80,
        128,
        53,
        144,
        96,
        32,
        1,
        53,
        97,
        1,
        128,
        86,
        91,
        97,
        0,
        127,
        96,
        4,
        128,
        54,
        3,
        96,
        64,
        129,
        16,
        21,
        97,
        0,
        221,
        87,
        96,
        0,
        128,
        253,
        91,
        80,
        128,
        53,
        144,
        96,
        32,
        1,
        53,
        97,
        1,
        163,
        86,
        91,
        97,
        0,
        127,
        96,
        4,
        128,
        54,
        3,
        96,
        64,
        129,
        16,
        21,
        97,
        1,
        0,
        87,
        96,
        0,
        128,
        253,
        91,
        80,
        128,
        53,
        144,
        96,
        32,
        1,
        53,
        97,
        1,
        214,
        86,
        91,
        96,
        0,
        97,
        1,
        25,
        131,
        131,
        97,
        2,
        9,
        86,
        91,
        144,
        80,
        129,
        131,
        1,
        129,
        20,
        97,
        1,
        38,
        87,
        254,
        91,
        130,
        129,
        16,
        21,
        128,
        21,
        97,
        1,
        54,
        87,
        80,
        129,
        129,
        16,
        21,
        91,
        97,
        1,
        60,
        87,
        254,
        91,
        80,
        80,
        80,
        86,
        91,
        96,
        0,
        97,
        1,
        77,
        131,
        131,
        97,
        2,
        31,
        86,
        91,
        144,
        80,
        129,
        131,
        2,
        129,
        20,
        97,
        1,
        90,
        87,
        254,
        91,
        130,
        21,
        128,
        97,
        1,
        101,
        87,
        80,
        129,
        21,
        91,
        128,
        97,
        1,
        54,
        87,
        80,
        130,
        129,
        16,
        21,
        128,
        21,
        97,
        1,
        54,
        87,
        80,
        129,
        129,
        16,
        21,
        97,
        1,
        60,
        87,
        254,
        91,
        96,
        0,
        97,
        1,
        140,
        131,
        131,
        97,
        2,
        67,
        86,
        91,
        144,
        80,
        129,
        131,
        3,
        129,
        20,
        97,
        1,
        153,
        87,
        254,
        91,
        130,
        129,
        17,
        21,
        97,
        1,
        60,
        87,
        254,
        91,
        96,
        0,
        97,
        1,
        175,
        131,
        131,
        97,
        2,
        83,
        86,
        91,
        144,
        80,
        129,
        131,
        1,
        129,
        20,
        97,
        1,
        188,
        87,
        254,
        91,
        96,
        0,
        130,
        18,
        97,
        1,
        205,
        87,
        130,
        129,
        18,
        21,
        97,
        1,
        54,
        86,
        91,
        130,
        129,
        18,
        97,
        1,
        60,
        87,
        254,
        91,
        96,
        0,
        97,
        1,
        226,
        131,
        131,
        97,
        2,
        105,
        86,
        91,
        144,
        80,
        129,
        131,
        3,
        129,
        20,
        97,
        1,
        239,
        87,
        254,
        91,
        96,
        0,
        130,
        18,
        97,
        2,
        0,
        87,
        130,
        129,
        19,
        21,
        97,
        1,
        54,
        86,
        91,
        130,
        129,
        19,
        97,
        1,
        60,
        87,
        254,
        91,
        128,
        130,
        1,
        130,
        129,
        16,
        21,
        97,
        2,
        25,
        87,
        96,
        0,
        128,
        253,
        91,
        146,
        145,
        80,
        80,
        86,
        91,
        96,
        0,
        130,
        21,
        128,
        97,
        2,
        58,
        87,
        80,
        80,
        129,
        129,
        2,
        129,
        131,
        130,
        129,
        97,
        2,
        55,
        87,
        254,
        91,
        4,
        20,
        91,
        97,
        2,
        25,
        87,
        96,
        0,
        128,
        253,
        91,
        128,
        130,
        3,
        130,
        129,
        17,
        21,
        97,
        2,
        25,
        87,
        96,
        0,
        128,
        253,
        91,
        129,
        129,
        1,
        130,
        129,
        18,
        21,
        96,
        0,
        131,
        18,
        21,
        20,
        97,
        2,
        25,
        87,
        96,
        0,
        128,
        253,
        91,
        128,
        130,
        3,
        130,
        129,
        19,
        21,
        96,
        0,
        131,
        18,
        21,
        20,
        97,
        2,
        25,
        87,
        96,
        0,
        128,
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
        16,
        137,
        195,
        226,
        47,
        225,
        174,
        7,
        68,
        241,
        247,
        45,
        241,
        232,
        18,
        38,
        254,
        85,
        171,
        76,
        126,
        105,
        100,
        77,
        122,
        100,
        78,
        120,
        178,
        214,
        10,
        250,
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
    pub static LOWGASSAFEMATHECHIDNATEST_BYTECODE: ::ethers::core::types::Bytes =
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
        87,
        87,
        96,
        0,
        53,
        96,
        224,
        28,
        128,
        99,
        15,
        147,
        80,
        0,
        20,
        97,
        0,
        92,
        87,
        128,
        99,
        15,
        228,
        140,
        92,
        20,
        97,
        0,
        129,
        87,
        128,
        99,
        109,
        136,
        111,
        174,
        20,
        97,
        0,
        164,
        87,
        128,
        99,
        146,
        57,
        231,
        119,
        20,
        97,
        0,
        199,
        87,
        128,
        99,
        189,
        120,
        178,
        13,
        20,
        97,
        0,
        234,
        87,
        91,
        96,
        0,
        128,
        253,
        91,
        97,
        0,
        127,
        96,
        4,
        128,
        54,
        3,
        96,
        64,
        129,
        16,
        21,
        97,
        0,
        114,
        87,
        96,
        0,
        128,
        253,
        91,
        80,
        128,
        53,
        144,
        96,
        32,
        1,
        53,
        97,
        1,
        13,
        86,
        91,
        0,
        91,
        97,
        0,
        127,
        96,
        4,
        128,
        54,
        3,
        96,
        64,
        129,
        16,
        21,
        97,
        0,
        151,
        87,
        96,
        0,
        128,
        253,
        91,
        80,
        128,
        53,
        144,
        96,
        32,
        1,
        53,
        97,
        1,
        65,
        86,
        91,
        97,
        0,
        127,
        96,
        4,
        128,
        54,
        3,
        96,
        64,
        129,
        16,
        21,
        97,
        0,
        186,
        87,
        96,
        0,
        128,
        253,
        91,
        80,
        128,
        53,
        144,
        96,
        32,
        1,
        53,
        97,
        1,
        128,
        86,
        91,
        97,
        0,
        127,
        96,
        4,
        128,
        54,
        3,
        96,
        64,
        129,
        16,
        21,
        97,
        0,
        221,
        87,
        96,
        0,
        128,
        253,
        91,
        80,
        128,
        53,
        144,
        96,
        32,
        1,
        53,
        97,
        1,
        163,
        86,
        91,
        97,
        0,
        127,
        96,
        4,
        128,
        54,
        3,
        96,
        64,
        129,
        16,
        21,
        97,
        1,
        0,
        87,
        96,
        0,
        128,
        253,
        91,
        80,
        128,
        53,
        144,
        96,
        32,
        1,
        53,
        97,
        1,
        214,
        86,
        91,
        96,
        0,
        97,
        1,
        25,
        131,
        131,
        97,
        2,
        9,
        86,
        91,
        144,
        80,
        129,
        131,
        1,
        129,
        20,
        97,
        1,
        38,
        87,
        254,
        91,
        130,
        129,
        16,
        21,
        128,
        21,
        97,
        1,
        54,
        87,
        80,
        129,
        129,
        16,
        21,
        91,
        97,
        1,
        60,
        87,
        254,
        91,
        80,
        80,
        80,
        86,
        91,
        96,
        0,
        97,
        1,
        77,
        131,
        131,
        97,
        2,
        31,
        86,
        91,
        144,
        80,
        129,
        131,
        2,
        129,
        20,
        97,
        1,
        90,
        87,
        254,
        91,
        130,
        21,
        128,
        97,
        1,
        101,
        87,
        80,
        129,
        21,
        91,
        128,
        97,
        1,
        54,
        87,
        80,
        130,
        129,
        16,
        21,
        128,
        21,
        97,
        1,
        54,
        87,
        80,
        129,
        129,
        16,
        21,
        97,
        1,
        60,
        87,
        254,
        91,
        96,
        0,
        97,
        1,
        140,
        131,
        131,
        97,
        2,
        67,
        86,
        91,
        144,
        80,
        129,
        131,
        3,
        129,
        20,
        97,
        1,
        153,
        87,
        254,
        91,
        130,
        129,
        17,
        21,
        97,
        1,
        60,
        87,
        254,
        91,
        96,
        0,
        97,
        1,
        175,
        131,
        131,
        97,
        2,
        83,
        86,
        91,
        144,
        80,
        129,
        131,
        1,
        129,
        20,
        97,
        1,
        188,
        87,
        254,
        91,
        96,
        0,
        130,
        18,
        97,
        1,
        205,
        87,
        130,
        129,
        18,
        21,
        97,
        1,
        54,
        86,
        91,
        130,
        129,
        18,
        97,
        1,
        60,
        87,
        254,
        91,
        96,
        0,
        97,
        1,
        226,
        131,
        131,
        97,
        2,
        105,
        86,
        91,
        144,
        80,
        129,
        131,
        3,
        129,
        20,
        97,
        1,
        239,
        87,
        254,
        91,
        96,
        0,
        130,
        18,
        97,
        2,
        0,
        87,
        130,
        129,
        19,
        21,
        97,
        1,
        54,
        86,
        91,
        130,
        129,
        19,
        97,
        1,
        60,
        87,
        254,
        91,
        128,
        130,
        1,
        130,
        129,
        16,
        21,
        97,
        2,
        25,
        87,
        96,
        0,
        128,
        253,
        91,
        146,
        145,
        80,
        80,
        86,
        91,
        96,
        0,
        130,
        21,
        128,
        97,
        2,
        58,
        87,
        80,
        80,
        129,
        129,
        2,
        129,
        131,
        130,
        129,
        97,
        2,
        55,
        87,
        254,
        91,
        4,
        20,
        91,
        97,
        2,
        25,
        87,
        96,
        0,
        128,
        253,
        91,
        128,
        130,
        3,
        130,
        129,
        17,
        21,
        97,
        2,
        25,
        87,
        96,
        0,
        128,
        253,
        91,
        129,
        129,
        1,
        130,
        129,
        18,
        21,
        96,
        0,
        131,
        18,
        21,
        20,
        97,
        2,
        25,
        87,
        96,
        0,
        128,
        253,
        91,
        128,
        130,
        3,
        130,
        129,
        19,
        21,
        96,
        0,
        131,
        18,
        21,
        20,
        97,
        2,
        25,
        87,
        96,
        0,
        128,
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
        16,
        137,
        195,
        226,
        47,
        225,
        174,
        7,
        68,
        241,
        247,
        45,
        241,
        232,
        18,
        38,
        254,
        85,
        171,
        76,
        126,
        105,
        100,
        77,
        122,
        100,
        78,
        120,
        178,
        214,
        10,
        250,
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
    pub static LOWGASSAFEMATHECHIDNATEST_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct LowGasSafeMathEchidnaTest<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for LowGasSafeMathEchidnaTest<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for LowGasSafeMathEchidnaTest<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for LowGasSafeMathEchidnaTest<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for LowGasSafeMathEchidnaTest<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(LowGasSafeMathEchidnaTest))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> LowGasSafeMathEchidnaTest<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                LOWGASSAFEMATHECHIDNATEST_ABI.clone(),
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
                LOWGASSAFEMATHECHIDNATEST_ABI.clone(),
                LOWGASSAFEMATHECHIDNATEST_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `checkAdd` (0x0f935000) function
        pub fn check_add(
            &self,
            x: ::ethers::core::types::U256,
            y: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([15, 147, 80, 0], (x, y))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `checkAddi` (0x9239e777) function
        pub fn check_addi(
            &self,
            x: ::ethers::core::types::I256,
            y: ::ethers::core::types::I256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([146, 57, 231, 119], (x, y))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `checkMul` (0x0fe48c5c) function
        pub fn check_mul(
            &self,
            x: ::ethers::core::types::U256,
            y: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([15, 228, 140, 92], (x, y))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `checkSub` (0x6d886fae) function
        pub fn check_sub(
            &self,
            x: ::ethers::core::types::U256,
            y: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([109, 136, 111, 174], (x, y))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `checkSubi` (0xbd78b20d) function
        pub fn check_subi(
            &self,
            x: ::ethers::core::types::I256,
            y: ::ethers::core::types::I256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([189, 120, 178, 13], (x, y))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
        for LowGasSafeMathEchidnaTest<M>
    {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `checkAdd` function with signature `checkAdd(uint256,uint256)` and selector `0x0f935000`
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
    #[ethcall(name = "checkAdd", abi = "checkAdd(uint256,uint256)")]
    pub struct CheckAddCall {
        pub x: ::ethers::core::types::U256,
        pub y: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `checkAddi` function with signature `checkAddi(int256,int256)` and selector `0x9239e777`
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
    #[ethcall(name = "checkAddi", abi = "checkAddi(int256,int256)")]
    pub struct CheckAddiCall {
        pub x: ::ethers::core::types::I256,
        pub y: ::ethers::core::types::I256,
    }
    ///Container type for all input parameters for the `checkMul` function with signature `checkMul(uint256,uint256)` and selector `0x0fe48c5c`
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
    #[ethcall(name = "checkMul", abi = "checkMul(uint256,uint256)")]
    pub struct CheckMulCall {
        pub x: ::ethers::core::types::U256,
        pub y: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `checkSub` function with signature `checkSub(uint256,uint256)` and selector `0x6d886fae`
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
    #[ethcall(name = "checkSub", abi = "checkSub(uint256,uint256)")]
    pub struct CheckSubCall {
        pub x: ::ethers::core::types::U256,
        pub y: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `checkSubi` function with signature `checkSubi(int256,int256)` and selector `0xbd78b20d`
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
    #[ethcall(name = "checkSubi", abi = "checkSubi(int256,int256)")]
    pub struct CheckSubiCall {
        pub x: ::ethers::core::types::I256,
        pub y: ::ethers::core::types::I256,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum LowGasSafeMathEchidnaTestCalls {
        CheckAdd(CheckAddCall),
        CheckAddi(CheckAddiCall),
        CheckMul(CheckMulCall),
        CheckSub(CheckSubCall),
        CheckSubi(CheckSubiCall),
    }
    impl ::ethers::core::abi::AbiDecode for LowGasSafeMathEchidnaTestCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <CheckAddCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::CheckAdd(decoded));
            }
            if let Ok(decoded) = <CheckAddiCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::CheckAddi(decoded));
            }
            if let Ok(decoded) = <CheckMulCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::CheckMul(decoded));
            }
            if let Ok(decoded) = <CheckSubCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::CheckSub(decoded));
            }
            if let Ok(decoded) = <CheckSubiCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::CheckSubi(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for LowGasSafeMathEchidnaTestCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::CheckAdd(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::CheckAddi(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::CheckMul(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::CheckSub(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::CheckSubi(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for LowGasSafeMathEchidnaTestCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::CheckAdd(element) => ::core::fmt::Display::fmt(element, f),
                Self::CheckAddi(element) => ::core::fmt::Display::fmt(element, f),
                Self::CheckMul(element) => ::core::fmt::Display::fmt(element, f),
                Self::CheckSub(element) => ::core::fmt::Display::fmt(element, f),
                Self::CheckSubi(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<CheckAddCall> for LowGasSafeMathEchidnaTestCalls {
        fn from(value: CheckAddCall) -> Self {
            Self::CheckAdd(value)
        }
    }
    impl ::core::convert::From<CheckAddiCall> for LowGasSafeMathEchidnaTestCalls {
        fn from(value: CheckAddiCall) -> Self {
            Self::CheckAddi(value)
        }
    }
    impl ::core::convert::From<CheckMulCall> for LowGasSafeMathEchidnaTestCalls {
        fn from(value: CheckMulCall) -> Self {
            Self::CheckMul(value)
        }
    }
    impl ::core::convert::From<CheckSubCall> for LowGasSafeMathEchidnaTestCalls {
        fn from(value: CheckSubCall) -> Self {
            Self::CheckSub(value)
        }
    }
    impl ::core::convert::From<CheckSubiCall> for LowGasSafeMathEchidnaTestCalls {
        fn from(value: CheckSubiCall) -> Self {
            Self::CheckSubi(value)
        }
    }
}
