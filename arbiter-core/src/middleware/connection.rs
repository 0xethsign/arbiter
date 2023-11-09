//! Messengers/connections to the underlying EVM in the environment.

use super::*;
use std::{
    collections::HashMap,
    fmt::Debug,
    pin::Pin,
    sync::{Arc, Mutex, Weak},
    task::Poll,
};

use crossbeam_channel::TryRecvError;
use ethers::{
    prelude::ProviderError,
    providers::{JsonRpcClient, PubsubClient},
    types::{Filter, FilteredParams},
};
use futures_util::{stream, Stream};
use serde::{de::DeserializeOwned, Serialize};
use serde_json::value::RawValue;

use super::cast::revm_logs_to_ethers_logs;
use crate::environment::{
    Broadcast, EventBroadcaster, InstructionSender, OutcomeReceiver, OutcomeSender,
};

/// Represents a connection to the EVM contained in the corresponding
/// [`Environment`].
#[derive(Debug)]
pub struct Connection {
    /// Used to send calls and transactions to the [`Environment`] to be
    /// executed by `revm`.
    pub(crate) instruction_sender: Weak<InstructionSender>,

    /// Used to send results back to a client that made a call/transaction with
    /// the [`Environment`]. This [`ResultSender`] is passed along with a
    /// call/transaction so the [`Environment`] can reply back with the
    /// [`ExecutionResult`].
    pub(crate) outcome_sender: OutcomeSender,

    /// Used to receive the [`ExecutionResult`] from the [`Environment`] upon
    /// call/transact.
    pub(crate) outcome_receiver: OutcomeReceiver,

    /// A reference to the [`EventBroadcaster`] so that more receivers of the
    /// broadcast can be taken from it.
    pub(crate) event_broadcaster: Arc<Mutex<EventBroadcaster>>,

    /// A collection of `FilterReceiver`s that will receive outgoing logs
    /// generated by `revm` and output by the [`Environment`].
    pub(crate) filter_receivers: Arc<Mutex<HashMap<ethers::types::U256, FilterReceiver>>>,
}

#[async_trait::async_trait]
impl JsonRpcClient for Connection {
    type Error = ProviderError;

    /// Processes a JSON-RPC request and returns the response.
    /// Currently only handles the `eth_getFilterChanges` call since this is
    /// used for polling events emitted from the [`Environment`].
    async fn request<T: Serialize + Send + Sync, R: DeserializeOwned>(
        &self,
        method: &str,
        params: T,
    ) -> Result<R, ProviderError> {
        match method {
            "eth_getFilterChanges" => {
                // TODO: The extra json serialization/deserialization can probably be avoided
                // somehow

                trace!("Getting filter changes...");
                // Get the `Filter` ID from the params `T`
                // First convert it into a JSON `Value`
                let value = serde_json::to_value(&params)?;

                // Take this value as an array then cast it to a string
                let str = value.as_array().ok_or(ProviderError::CustomError(
                    "The params value passed to the `Connection` via a `request` was empty. 
                    This is likely due to not specifying a specific `Filter` ID!".to_string()
                ))?[0]
                    .as_str().ok_or(ProviderError::CustomError(
                        "The params value passed to the `Connection` via a `request` could not be later cast to `str`!".to_string()
                    ))?;

                // Now get the `U256` ID via the string decoded from hex radix.
                let id = ethers::types::U256::from_str_radix(str, 16)
                    .map_err(|e| ProviderError::CustomError(
                        format!("The `str` representation of the filter ID could not be cast into `U256` due to: {:?}!", 
                        e)))?;

                // Get the corresponding `filter_receiver` and await for logs to appear.
                let mut filter_receivers = self.filter_receivers.lock().unwrap();
                let filter_receiver =
                    filter_receivers
                        .get_mut(&id)
                        .ok_or(ProviderError::CustomError(
                            "The filter ID does not seem to match any that this client owns!"
                                .to_string(),
                        ))?;
                let mut logs = vec![];
                let filtered_params = FilteredParams::new(Some(filter_receiver.filter.clone()));
                if let Ok(broadcast) = filter_receiver.receiver.try_recv() {
                    match broadcast {
                        Broadcast::Event(received_logs) => {
                            let ethers_logs = revm_logs_to_ethers_logs(received_logs);
                            for log in ethers_logs {
                                if filtered_params.filter_address(&log)
                                    && filtered_params.filter_topics(&log)
                                {
                                    logs.push(log);
                                }
                            }
                        }
                        Broadcast::StopSignal => {
                            return Err(ProviderError::CustomError(
                                "The `EventBroadcaster` has stopped!".to_string(),
                            ))
                        }
                    }
                }
                // Take the logs and Stringify then JSONify to cast into `R`.
                let logs_str = serde_json::to_string(&logs)?;
                let logs_deserializeowned: R = serde_json::from_str(&logs_str)?;
                Ok(logs_deserializeowned)
            }
            val => Err(ProviderError::CustomError(format!(
                "The method `{}` is not supported by the `Connection`!",
                val
            ))),
        }
    }
}

impl PubsubClient for Connection {
    type NotificationStream = Pin<Box<dyn Stream<Item = Box<RawValue>> + Send>>;

    fn subscribe<T: Into<ethers::types::U256>>(
        &self,
        id: T,
    ) -> Result<Self::NotificationStream, Self::Error> {
        let id = id.into();
        debug!("Subscribing to filter with ID: {:?}", id);

        let filter_receiver = self.filter_receivers.lock().unwrap().get(&id).cloned();
        match filter_receiver {
            Some(filter_receiver) => {
                let stream = stream::poll_fn(move |cx| {
                    match filter_receiver.receiver.try_recv() {
                        Ok(Broadcast::Event(logs)) => {
                            let filtered_params =
                                FilteredParams::new(Some(filter_receiver.filter.clone()));
                            let ethers_logs = revm_logs_to_ethers_logs(logs);
                            // Return the first log that matches the filter, if any
                            for log in ethers_logs {
                                if filtered_params.filter_address(&log)
                                    && filtered_params.filter_topics(&log)
                                {
                                    let raw_log = match serde_json::to_string(&log) {
                                        Ok(log) => log,
                                        Err(e) => {
                                            eprintln!("Error serializing log: {}", e);
                                            continue;
                                        }
                                    };
                                    let raw_log = match RawValue::from_string(raw_log) {
                                        Ok(log) => log,
                                        Err(e) => {
                                            eprintln!("Error creating RawValue: {}", e);
                                            continue;
                                        }
                                    };
                                    return Poll::Ready(Some(raw_log));
                                }
                            }
                            Poll::Ready(None)
                        }
                        Ok(Broadcast::StopSignal) => {
                            eprintln!("The `EventBroadcaster` has stopped!");
                            Poll::Ready(None)
                        }
                        Err(TryRecvError::Empty) => {
                            // No logs available yet, so we'll try again later
                            cx.waker().wake_by_ref();
                            Poll::Pending
                        }
                        Err(TryRecvError::Disconnected) => {
                            eprintln!("The `EventBroadcaster` has been disconnected!");
                            Poll::Ready(None)
                        }
                    }
                });
                Ok(Box::pin(stream))
            }
            None => Err(ProviderError::CustomError(
                "The filter ID does not seem to match any that this client owns!".to_string(),
            )),
        }
    }

    fn unsubscribe<T: Into<ethers::types::U256>>(&self, id: T) -> Result<(), Self::Error> {
        let id = id.into();
        debug!("Unsubscribing from filter with ID: {:?}", id);
        let mut filter_receivers = self.filter_receivers.lock().unwrap();
        if filter_receivers.remove(&id).is_some() {
            Ok(())
        } else {
            Err(ProviderError::CustomError(
                "The filter ID does not seem to match any that this client owns!".to_string(),
            ))
        }
    }
}

/// Packages together a [`crossbeam_channel::Receiver<Vec<Log>>`] along with a
/// [`Filter`] for events. Allows the client to have a stream of filtered
/// events.
#[derive(Clone, Debug)]
pub(crate) struct FilterReceiver {
    /// The filter definition used for this receiver.
    /// Comes from the `ethers-rs` crate.
    pub(crate) filter: Filter,

    /// The receiver for the channel that receives logs from the broadcaster.
    /// These are filtered upon reception.
    pub(crate) receiver: crossbeam_channel::Receiver<Broadcast>,
}
