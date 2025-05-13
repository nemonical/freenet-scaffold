use freenet_stdlib::prelude::*;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;

use crate::ComposableState;

pub struct Contract<T>(std::marker::PhantomData<T>)
where
    T: ComposableState;

impl<T> Contract<T>
where
    T: ComposableState<ParentState = RefCell<RelatedContracts<'static>>>
        + Serialize
        + for<'a> Deserialize<'a>,
{
    // TODO: Is there a better way to give `ComposableState`s the ability to get related contracts
    // than setting its `ParentState` to `RefCell<RelatedContracts<'static>>` and giving it dummy
    // data in every function except `validate_state()`?
    // Maybe it should be `ParentState = Option<RefCell<RelatedContracts<'static>>>`?
    // Is there a good way to let the user decide ParentState themselves and still get the benefit
    // of this function?

    pub fn validate_state(
        parameters: Parameters<'static>,
        state: State<'static>,
        related: RelatedContracts<'static>,
    ) -> Result<ValidateResult, ContractError> {
        let related = RefCell::new(related);

        match T::verify(
            &match serde_json::from_slice(state.as_ref()) {
                Ok(state) => state,
                Err(error) => {
                    return Err(ContractError::Deser(format!(
                        "an error occured while deserializing the contract state: {}",
                        error
                    )))
                }
            },
            &related,
            &match serde_json::from_slice(parameters.as_ref()) {
                Ok(parameters) => parameters,
                Err(error) => {
                    return Err(ContractError::Deser(format!(
                        "an error occured while deserializing the contract parameters: {}",
                        error
                    )))
                }
            },
        ) {
            Ok(_) => {
                let missing_contracts = related
                    .borrow()
                    .states()
                    .filter_map(|(id, state)| {
                        if state.is_none() {
                            Some(id.to_owned())
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>();
                if missing_contracts.is_empty() {
                    Ok(ValidateResult::Valid)
                } else {
                    Ok(ValidateResult::RequestRelated(missing_contracts))
                }
            }
            Err(_) => Ok(ValidateResult::Invalid),
        }
    }
}

impl<T, P> Contract<T>
where
    T: ComposableState<ParentState = P> + Serialize + for<'a> Deserialize<'a>,
    P: Default,
{
    pub fn summarize_state(
        parameters: Parameters<'static>,
        state: State<'static>,
    ) -> Result<StateSummary<'static>, ContractError> {
        match serde_json::to_vec(&T::summarize(
            &match serde_json::from_slice(state.as_ref()) {
                Ok(state) => state,
                Err(error) => {
                    return Err(ContractError::Deser(format!(
                        "an error occured while deserializing the contract state: {}",
                        error
                    )))
                }
            },
            &T::ParentState::default(),
            &match serde_json::from_slice(parameters.as_ref()) {
                Ok(parameters) => parameters,
                Err(error) => {
                    return Err(ContractError::Deser(format!(
                        "an error occured while deserializing the contract parameters: {}",
                        error
                    )))
                }
            },
        )) {
            Ok(summary) => Ok(summary.into()),
            Err(_) => todo!(),
        }
    }

    pub fn get_state_delta(
        parameters: Parameters<'static>,
        state: State<'static>,
        summary: StateSummary<'static>,
    ) -> Result<StateDelta<'static>, ContractError> {
        match serde_json::to_vec(&T::delta(
            &match serde_json::from_slice(state.as_ref()) {
                Ok(state) => state,
                Err(error) => {
                    return Err(ContractError::Deser(format!(
                        "an error occured while deserializing the contract state: {}",
                        error
                    )))
                }
            },
            &T::ParentState::default(),
            &match serde_json::from_slice(parameters.as_ref()) {
                Ok(parameters) => parameters,
                Err(error) => {
                    return Err(ContractError::Deser(format!(
                        "an error occured while deserializing the contract parameters: {}",
                        error
                    )))
                }
            },
            &match serde_json::from_slice(summary.as_ref()) {
                Ok(summary) => summary,
                Err(error) => {
                    return Err(ContractError::Deser(format!(
                        "an error occurred while deserializing the contract summary: {}",
                        error
                    )))
                }
            },
        )) {
            Ok(delta) => Ok(delta.into()),
            Err(_) => todo!(),
        }
    }

    pub fn update_state(
        parameters: Parameters<'static>,
        state: State<'static>,
        data: Vec<UpdateData<'static>>,
    ) -> Result<UpdateModification<'static>, ContractError> {
        let mut state: T = match serde_json::from_slice(state.as_ref()) {
            Ok(state) => state,
            Err(error) => {
                return Err(ContractError::Deser(format!(
                    "an error occured while deserializing the contract state: {}",
                    error
                )))
            }
        };

        todo!()
    }
}
