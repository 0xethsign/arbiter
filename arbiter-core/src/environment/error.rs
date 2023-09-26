//! Errors that can occur when managing or interfacing with Arbiter's sandboxed
//! Ethereum environment.

use super::*;

/// Errors that can occur when managing or interfacing with the Ethereum
/// environment.
///
/// ## What are we trying to catch?
/// The errors here are at a fairly low level and should be quite rare (if
/// possible). Errors that come from smart contracts (e.g., reverts or halts)
/// will not be caught here and will instead carried out into the
/// [`RevmMiddleware`]. Please bring up if you catch errors here by sending a
/// message in the [Telegram group](https://t.me/arbiter_rs) or on
/// [GitHub](https://github.com/primitivefinance/arbiter/).
#[derive(Error, Debug, Clone)]
pub enum EnvironmentError {
    /// [`EnvironmentError::Execution`] is thrown when the [`EVM`] itself
    /// throws an error in execution. To be clear, this is not a contract
    /// revert or halt, this is likely an error in `revm`. Please report
    /// this type of error.
    #[error("execution error! the source error is: {0:?}")]
    Execution(EVMError<Infallible>),

    /// [`EnvironmentError::Transaction`] is thrown when a transaction fails
    /// to be processed by the [`EVM`]. This could be due to a insufficient
    /// funds to pay for gas, an invalid nonce, or other reasons. This error
    /// can be quite common and should be handled gracefully.
    #[error("transaction error! the source error is: {0:?}")]
    Transaction(InvalidTransaction),

    /// [`EnvironmentError::Account`] is thrown when there is an issue handling
    /// accounts in the [`EVM`]. This could be due to an account already
    /// existing or other reasons.
    #[error("account error! due to: {0:?}")]
    Account(String),

    /// [`EnvironmentError::Pause`] is thrown when the [`Environment`]
    /// fails to pause. This should likely never occur, but if it does,
    /// please report this error!
    #[error("error pausing! due to: {0:?}")]
    Pause(String),

    /// [`EnvironmentError::Stop`] is thrown when the [`Environment`]
    /// fails to stop. This error could occur due to an invalid state transition
    /// or other unexpected conditions. If this error is thrown, it indicates
    /// a serious issue that needs to be investigated. Please report this error!
    #[error("error stopping! due to: {0:?}")]
    Stop(String),

    /// [`EnvironmentError::Communication`] is thrown when a channel for
    /// receiving or broadcasting fails in some way. This error could happen
    /// due to a channel being closed accidentally. If this is thrown, a
    /// restart of the simulation and an investigation into what caused a
    /// dropped channel is necessary.
    #[error("error communicating! due to: {0}")]
    Communication(String),

    /// [`EnvironmentError::Broadcast`] is thrown when the
    /// [`EventBroadcaster`] fails to broadcast events. This should be
    /// rare (if not impossible). If this is thrown, please report this error!
    #[error("error broadcasting! the source error is: {0}")]
    Broadcast(#[from] crossbeam_channel::SendError<Vec<Log>>),

    /// [`EnvironmentError::Conversion`] is thrown when a type fails to
    /// convert into another (typically a type used in `revm` versus a type used
    /// in [`ethers-rs`](https://github.com/gakonst/ethers-rs)).
    /// This error should be rare (if not impossible).
    /// Furthermore, after a switch to [`alloy`](https://github.com/alloy-rs)
    /// this will be (hopefully) unnecessary!
    #[error("conversion error! the source error is: {0}")]
    Conversion(String),

    /// [`EnvironmentError::TransactionReceivedWhilePaused`] is thrown when
    /// a transaction is received while the [`Environment`] is paused.
    /// This can be quite common due to concurrency issues, but should be
    /// handled gracefully.
    #[error("transaction was received while the environment was paused. this transaction was not processed.")]
    TransactionReceivedWhilePaused,

    /// [`EnvironmentError::NotUserControlledGasSettings`] is thrown when the
    /// [`Environment`] is not in a [`GasSettings::UserControlled`] state and
    /// an attempt is made to externally change the gas price.
    #[error("error in the environment! attempted to set a gas price when the `GasSettings` is not `GasSettings::UserControlled`")]
    NotUserControlledGasSettings,

    /// [`EnvironmentError::NotUserControlledBlockType`] is thrown when
    /// the [`Environment`] is in a [`BlockType::RandomlySampled`] state and
    /// an attempt is made to externally change the block number and timestamp.
    #[error("error in the environment! attempted to externally change block number and timestamp when `BlockType` is not `BlockType::UserControlled`.")]
    NotUserControlledBlockType,

    /// [`EnvironmentError::NotRandomlySampledBlockType`] is thrown when
    /// the [`Environment`] is **not** in a [`BlockType::RandomlySampled`] state
    /// and an attempt is made to set the gas price via a multiplier.
    /// That is, the user has chosen [`GasSettings::RandomlySampled`] without
    /// [`BlockType::RandomlySampled`].
    #[error("error in the environment! attempted to set a gas price via a multiplier when the `BlockType` is not `BlockType::RandomlySampled`.")]
    NotRandomlySampledBlockType,
}
