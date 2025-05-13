use freenet_stdlib::prelude::*;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;

use crate::ComposableState;

pub struct Contract<T>(std::marker::PhantomData<T>)
    where T: ComposableState;

impl<T> Contract<T> 
where
    T: ComposableState<ParentState = RefCell<RelatedContracts<'static>>> + Serialize + for<'a> Deserialize<'a>
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
        match T::verify(
            &match serde_json::from_slice(state.as_ref()) {
                Ok(state) => state,
                // TODO: write an error message here?
                Err(error) => return Err(ContractError::Deser(format!("{}", error))),
            },
            &RefCell::new(related),
            &match serde_json::from_slice(parameters.as_ref()) {
                Ok(parameters) => parameters,
                // TODO: write an error message here?
                Err(error) => return Err(ContractError::Deser(format!("{}", error)))
            },
        ) {
            Ok(_) => Ok(ValidateResult::Valid),
            Err(_) => Ok(ValidateResult::Invalid),
        }
    }
}

impl<T, P> Contract<T>
where
    T: ComposableState<ParentState = P>,
    P: Default,
{ 
    pub fn summarize_state(
        parameters: Parameters<'static>,
        state: State<'static>,
    ) -> Result<StateSummary<'static>, ContractError> {
        match serde_json::to_vec(&T::summarize(todo!(), &T::ParentState::default(), todo!())) {
            Ok(summary) => Ok(summary.into()),
            Err(_) => todo!(),
        }
    }

    pub fn get_state_delta(
        parameters: Parameters<'static>,
        state: State<'static>,
        summary: StateSummary<'static>,
    ) -> Result<StateDelta<'static>, ContractError> {
        match serde_json::to_vec(&T::delta(todo!(), &T::ParentState::default(), todo!(), todo!())) {
            Ok(delta) => Ok(delta.into()),
            Err(_) => todo!(),
        }
    }

    pub fn update_state(
        parameters: Parameters<'static>,
        state: State<'static>,
        data: Vec<UpdateData<'static>>,
    ) -> Result<UpdateModification<'static>, ContractError> {
        match T::apply_delta(todo!(), &T::ParentState::default(), todo!(), todo!()) {
            Ok(()) => todo!(),
            Err(_) => todo!(),
        }
    }
}
