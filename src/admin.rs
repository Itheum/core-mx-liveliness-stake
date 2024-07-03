use core_mx_life_bonding_sc::{
    errors::{ERR_ALREADY_ACTIVE, ERR_ALREADY_INACTIVE, ERR_NOT_PRIVILEGED},
    only_privileged,
};

use crate::{
    config::{self, State},
    events,
};

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[multiversx_sc::module]
pub trait AdminModule: config::ConfigModule + events::EventsModule {
    #[endpoint(setContractStateActive)]
    fn set_contract_state_active(&self) {
        only_privileged!(self, ERR_NOT_PRIVILEGED);
        require!(
            self.contract_state().get() == State::Inactive,
            ERR_ALREADY_ACTIVE
        );
        self.contract_state().set(State::Active);
        self.contract_state_event(State::Active);
    }

    #[endpoint(setContractStateInactive)]
    fn set_contract_state_inactive(&self) {
        only_privileged!(self, ERR_NOT_PRIVILEGED);
        require!(
            self.contract_state().get() == State::Active,
            ERR_ALREADY_INACTIVE
        );
        self.contract_state().set(State::Inactive);
        self.contract_state_event(State::Inactive);
    }
}
