use crate::{config, contexts::base::StorageCache, events, rewards, storage};

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

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
}
