use core_mx_life_bonding_sc::errors::{
    ERR_ALREADY_ACTIVE, ERR_ALREADY_INACTIVE, ERR_NOT_PRIVILEGED,
};

use crate::{
    config::{self, State},
    contexts::base::StorageCache,
    events, only_privileged, rewards, storage,
};

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[multiversx_sc::module]
pub trait AdminModule:
    config::ConfigModule + events::EventsModule + storage::StorageModule + rewards::RewardsModule
{
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

    #[endpoint(setMaxApr)]
    fn set_max_apr(&self, max_apr: BigUint) {
        only_privileged!(self, ERR_NOT_PRIVILEGED);

        let mut storage_cache = StorageCache::new(self);
        self.generate_aggregated_rewards(&mut storage_cache);

        self.max_apr().set(max_apr);
    }

    #[endpoint(setRewardsTokenIdentifier)]
    fn set_rewards_token_identifier(&self, token_identifier: TokenIdentifier) {
        only_privileged!(self, ERR_NOT_PRIVILEGED);
        self.rewards_token_identifier().set(token_identifier);
        // self.rewards_token_identifier_event(token_identifier);
    }

    #[endpoint(setPerBlockRewardAmount)]
    fn set_per_block_rewards(&self, per_block_amount: BigUint) {
        only_privileged!(self, ERR_NOT_PRIVILEGED);

        let mut storage_cache = StorageCache::new(self);
        self.generate_aggregated_rewards(&mut storage_cache);

        self.rewards_per_block().set(per_block_amount);
        // self.rewards_per_block_event(rewards_per_block);
    }

    #[payable("*")]
    #[endpoint(topUpRewards)]
    fn top_up_rewards(&self) {
        only_privileged!(self, ERR_NOT_PRIVILEGED);
        let payment = self.call_value().single_esdt();

        require!(
            payment.token_identifier == self.rewards_token_identifier().get(),
            "Invalid token identifier"
        );

        let mut storage_cache = StorageCache::new(self);
        self.generate_aggregated_rewards(&mut storage_cache);

        self.rewards_reserve()
            .update(|value| *value += payment.amount);
    }

    #[endpoint(withdrawRewards)]
    fn withdraw_rewards(&self, amount: BigUint) {
        only_privileged!(self, ERR_NOT_PRIVILEGED);

        let mut storage_cache = StorageCache::new(self);
        self.generate_aggregated_rewards(&mut storage_cache);

        require!(
            storage_cache.rewards_reserve >= amount,
            "Insufficient rewards reserve"
        );

        self.rewards_reserve().update(|value| *value -= amount);
    }

    #[endpoint(startProduceRewards)]
    fn start_produce_rewards(&self) {
        only_privileged!(self, ERR_NOT_PRIVILEGED);
        self.rewards_state().set(State::Active);
        self.last_reward_block_nonce()
            .set(self.blockchain().get_block_nonce());
    }

    #[endpoint(endProduceRewards)]
    fn end_produce_rewards(&self) {
        only_privileged!(self, ERR_NOT_PRIVILEGED);

        let mut storage_cache = StorageCache::new(self);
        self.generate_aggregated_rewards(&mut storage_cache);

        self.rewards_state().set(State::Inactive);
    }
}
