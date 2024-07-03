use crate::config::State;

#[multiversx_sc::module]
pub trait EventsModule {
    #[event("set_administrator_event")]
    fn set_administrator_event(&self, #[indexed] administrator: &ManagedAddress);

    #[event("contract_state_event")]
    fn contract_state_event(&self, #[indexed] state: State);
}
