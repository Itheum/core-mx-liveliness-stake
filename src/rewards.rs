use crate::{
    config::{self, BLOCKS_IN_YEAR, DIVISION_SAFETY_CONST, MAX_PERCENT},
    contexts::base::StorageCache,
    events, storage,
};

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[multiversx_sc::module]
pub trait RewardsModule:
    storage::StorageModule + config::ConfigModule + events::EventsModule
{
    fn generate_aggregated_rewards(&self, storage_cache: &mut StorageCache<Self>) {
        let extra_rewards = self.calculate_rewards_since_last_allocation(storage_cache);

        if extra_rewards > BigUint::zero() {
            storage_cache.accumulated_rewards += &extra_rewards;
            storage_cache.rewards_reserve -= &extra_rewards;
        }
    }

    // not used (usefull to enforce a max APR)
    fn get_amount_apr_bounded(&self, amount: &BigUint) -> BigUint {
        let max_apr = self.max_apr().get();
        amount * &max_apr / MAX_PERCENT / BLOCKS_IN_YEAR
    }

    fn calculate_rewards_since_last_allocation(
        &self,
        storage_cache: &mut StorageCache<Self>,
    ) -> BigUint {
        let current_block_nonce = self.blockchain().get_block_nonce();

        if !self.can_produce_rewards() {
            return BigUint::zero();
        }

        if current_block_nonce <= storage_cache.last_reward_block_nonce {
            return BigUint::zero();
        }

        let block_nonce_diff = current_block_nonce - storage_cache.last_reward_block_nonce;

        storage_cache.last_reward_block_nonce = current_block_nonce;

        &storage_cache.rewards_per_block * block_nonce_diff
    }

    fn calculate_caller_share_in_rewards(
        self,
        caller: &ManagedAddress,
        storage_cache: &mut StorageCache<Self>,
        timestamp: Option<u64>,
    ) -> BigUint {
        let total_staked_amount = self
            .tx()
            .to(self.bond_contract_address().get())
            .typed(core_mx_life_bonding_sc::life_bonding_sc_proxy::LifeBondingContractProxy)
            .total_bond_amount()
            .returns(ReturnsResult)
            .sync_call();

        let user_stake_amount = self
            .tx()
            .to(self.bond_contract_address().get())
            .typed(core_mx_life_bonding_sc::life_bonding_sc_proxy::LifeBondingContractProxy)
            .get_address_bonds_total_value(caller)
            .returns(ReturnsResult)
            .sync_call();

        let liveliness_score = self
            .tx()
            .to(self.bond_contract_address().get())
            .typed(core_mx_life_bonding_sc::life_bonding_sc_proxy::LifeBondingContractProxy)
            .get_address_bonds_avg_score(caller, timestamp)
            .returns(ReturnsResult)
            .sync_call();

        if total_staked_amount == BigUint::zero() {
            return BigUint::zero();
        }

        if storage_cache.accumulated_rewards == BigUint::zero() {
            return BigUint::zero();
        }

        let user_share = user_stake_amount * DIVISION_SAFETY_CONST / total_staked_amount;

        let claimable_rewards = user_share / &storage_cache.accumulated_rewards; // accumulated_rewards has DIVISION_SAFETY applied

        liveliness_score * claimable_rewards / MAX_PERCENT
    }
}
