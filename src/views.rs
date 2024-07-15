use crate::{config, contexts::base::StorageCache, events, rewards, storage};

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[derive(TopDecode, TopEncode, TypeAbi, PartialEq, Debug)]
pub struct ContractDetails<M: ManagedTypeApi> {
    pub rewards_reserve: BigUint<M>,
    pub accumulated_rewards: BigUint<M>,
    pub rewards_token_identifier: TokenIdentifier<M>,
    pub rewards_per_block: BigUint<M>,
    pub rewards_per_share: BigUint<M>,
    pub administrator: ManagedAddress<M>,
    pub bond_contract_address: ManagedAddress<M>,
    pub last_reward_block_nonce: u64,
    pub max_apr: BigUint<M>,
}

#[multiversx_sc::module]
pub trait ViewsModule:
    rewards::RewardsModule + events::EventsModule + config::ConfigModule + storage::StorageModule
{
    #[view(claimableRewards)]
    fn claimable_rewards(
        &self,
        caller: ManagedAddress,
        opt_bypass_liveliness: Option<bool>,
    ) -> BigUint {
        let bypass_liveliness = opt_bypass_liveliness.unwrap_or(false);

        let mut storage_cache = StorageCache::new(self);

        self.generate_aggregated_rewards(&mut storage_cache);

        self.calculate_caller_share_in_rewards(&caller, &mut storage_cache, bypass_liveliness)
    }

    #[view(contractDetails)]
    fn contract_details(self) -> ContractDetails<Self::Api> {
        let mut storage_cache = StorageCache::new(self);

        self.generate_aggregated_rewards(&mut storage_cache);

        

        ContractDetails {
            rewards_reserve: storage_cache.rewards_reserve.clone(),
            accumulated_rewards: storage_cache.accumulated_rewards.clone(),
            rewards_token_identifier: storage_cache.rewards_token_identifier.clone(),
            rewards_per_block: storage_cache.rewards_per_block.clone(),
            last_reward_block_nonce: storage_cache.last_reward_block_nonce,
            rewards_per_share: storage_cache.rewards_per_share.clone(),
            max_apr: storage_cache.max_apr.clone(),
            administrator: self.administrator().get(),
            bond_contract_address: self.bond_contract_address().get(),
        }
    }
}
