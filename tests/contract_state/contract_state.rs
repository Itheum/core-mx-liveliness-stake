use multiversx_sc_scenario::{api::SingleTxApi, imports::*, multiversx_chain_vm::world_mock};

use crate::utils::BigIntDec;

use core_mx_life_bonding_sc::life_bonding_sc_proxy::LifeBondingContractProxy as life_bonding_proxy;
use core_mx_liveliness_stake::liveliness_stake_proxy::CoreMxLivelinessStakeProxy as liveliness_proxy;

pub const OWNER_ADDRESS: TestAddress = TestAddress::new("owner");
pub const ADMIN_ADDRESS: TestAddress = TestAddress::new("admin");
pub const LIVELINESS_STAKE_CONTRACT_ADDRESS: TestSCAddress =
    TestSCAddress::new("liveliness_stake_contract");
pub const BONDING_CONTRACT_ADDRESS: TestSCAddress = TestSCAddress::new("bonding_contract");

pub const LIVELINESS_CODE_PATH: MxscPath =
    MxscPath::new("output/core-mx-liveliness-stake.mxsc.json");

pub const BOND_CODE_PATH: MxscPath =
    MxscPath::new("test_external_contracts/core-mx-life-bonding-sc.mxsc.json");

pub const FIRST_USER_ADDRESS: TestAddress = TestAddress::new("first_user");
pub const SECOND_USER_ADDRESS: TestAddress = TestAddress::new("second_user");
pub const THIRD_USER_ADDRESS: TestAddress = TestAddress::new("third_user");

pub const ITHEUM_TOKEN_IDENTIFIER: TestTokenIdentifier = TestTokenIdentifier::new("ITHBB-avlv");
pub const ANOTHER_TOKEN_IDENTIFIER: TestTokenIdentifier = TestTokenIdentifier::new("ANOTDD-clvb5s");

pub const DEFAULT_DECIMALS: u32 = 18;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();

    blockchain.register_contract(
        LIVELINESS_CODE_PATH,
        core_mx_liveliness_stake::ContractBuilder,
    );

    blockchain.register_contract(BOND_CODE_PATH, core_mx_life_bonding_sc::ContractBuilder);

    blockchain
}

pub struct ContractState {
    pub world: ScenarioWorld,
}

impl ContractState {
    pub fn new() -> Self {
        let mut world = world();

        world
            .account(OWNER_ADDRESS)
            .nonce(1)
            .balance(BigIntDec::from(100u64, DEFAULT_DECIMALS))
            .esdt_balance(
                ITHEUM_TOKEN_IDENTIFIER,
                BigIntDec::from(100_000_000u64, DEFAULT_DECIMALS),
            );

        world
            .account(ADMIN_ADDRESS)
            .nonce(1)
            .balance(BigIntDec::from(100u64, DEFAULT_DECIMALS))
            .esdt_balance(
                ITHEUM_TOKEN_IDENTIFIER,
                BigIntDec::from(100_000_000u64, DEFAULT_DECIMALS),
            );

        world
            .account(FIRST_USER_ADDRESS)
            .nonce(1)
            .balance(BigIntDec::from(10u64, DEFAULT_DECIMALS))
            .esdt_balance(
                ITHEUM_TOKEN_IDENTIFIER,
                BigIntDec::from(1_000u64, DEFAULT_DECIMALS),
            );

        world
            .account(SECOND_USER_ADDRESS)
            .nonce(1)
            .balance(BigIntDec::from(10u64, DEFAULT_DECIMALS))
            .esdt_balance(
                ITHEUM_TOKEN_IDENTIFIER,
                BigIntDec::from(5_000u64, DEFAULT_DECIMALS),
            );

        world
            .account(THIRD_USER_ADDRESS)
            .nonce(1)
            .balance(BigIntDec::from(10u64, DEFAULT_DECIMALS))
            .esdt_balance(
                ITHEUM_TOKEN_IDENTIFIER,
                BigIntDec::from(10_000u64, DEFAULT_DECIMALS),
            );

        Self { world }
    }

    pub fn deploy(&mut self) {
        self.world
            .tx()
            .from(OWNER_ADDRESS)
            .typed(liveliness_proxy)
            .init()
            .code(LIVELINESS_CODE_PATH)
            .new_address(LIVELINESS_STAKE_CONTRACT_ADDRESS)
            .run();
    }

    pub fn set_administrator(
        &mut self,
        caller: TestAddress,
        administrator: TestAddress,
        expect: Option<ExpectError>,
    ) {
        if let Some(expect) = expect {
            self.world
                .tx()
                .from(caller)
                .to(LIVELINESS_STAKE_CONTRACT_ADDRESS)
                .typed(liveliness_proxy)
                .set_administrator(administrator)
                .with_result(expect)
                .run();
        } else {
            self.world
                .tx()
                .from(caller)
                .to(LIVELINESS_STAKE_CONTRACT_ADDRESS)
                .typed(liveliness_proxy)
                .set_administrator(administrator)
                .run();
        }
    }

    pub fn set_contract_state_active(&mut self, caller: TestAddress, expect: Option<ExpectError>) {
        if let Some(expect) = expect {
            self.world
                .tx()
                .from(caller)
                .to(LIVELINESS_STAKE_CONTRACT_ADDRESS)
                .typed(liveliness_proxy)
                .set_contract_state_active()
                .with_result(expect)
                .run();
        } else {
            self.world
                .tx()
                .from(caller)
                .to(LIVELINESS_STAKE_CONTRACT_ADDRESS)
                .typed(liveliness_proxy)
                .set_contract_state_active()
                .run();
        }
    }

    pub fn set_contract_state_inactive(
        &mut self,
        caller: TestAddress,
        expect: Option<ExpectError>,
    ) {
        if let Some(expect) = expect {
            self.world
                .tx()
                .from(caller)
                .to(LIVELINESS_STAKE_CONTRACT_ADDRESS)
                .typed(liveliness_proxy)
                .set_contract_state_inactive()
                .with_result(expect)
                .run();
        } else {
            self.world
                .tx()
                .from(caller)
                .to(LIVELINESS_STAKE_CONTRACT_ADDRESS)
                .typed(liveliness_proxy)
                .set_contract_state_inactive()
                .run();
        }
    }

    pub fn start_produce_rewards(&mut self, caller: TestAddress, expect: Option<ExpectError>) {
        if let Some(expect) = expect {
            self.world
                .tx()
                .from(caller)
                .to(LIVELINESS_STAKE_CONTRACT_ADDRESS)
                .typed(liveliness_proxy)
                .start_produce_rewards()
                .with_result(expect)
                .run();
        } else {
            self.world
                .tx()
                .from(caller)
                .to(LIVELINESS_STAKE_CONTRACT_ADDRESS)
                .typed(liveliness_proxy)
                .start_produce_rewards()
                .run();
        }
    }

    pub fn end_produce_rewards(&mut self, caller: TestAddress, expect: Option<ExpectError>) {
        if let Some(expect) = expect {
            self.world
                .tx()
                .from(caller)
                .to(LIVELINESS_STAKE_CONTRACT_ADDRESS)
                .typed(liveliness_proxy)
                .end_produce_rewards()
                .with_result(expect)
                .run();
        } else {
            self.world
                .tx()
                .from(caller)
                .to(LIVELINESS_STAKE_CONTRACT_ADDRESS)
                .typed(liveliness_proxy)
                .end_produce_rewards()
                .run();
        }
    }

    pub fn set_rewards_token_identifier(
        &mut self,
        caller: TestAddress,
        token_identifier: TestTokenIdentifier,
        expect: Option<ExpectError>,
    ) {
        if let Some(expect) = expect {
            self.world
                .tx()
                .from(caller)
                .to(LIVELINESS_STAKE_CONTRACT_ADDRESS)
                .typed(liveliness_proxy)
                .set_rewards_token_identifier(token_identifier)
                .with_result(expect)
                .run();
        } else {
            self.world
                .tx()
                .from(caller)
                .to(LIVELINESS_STAKE_CONTRACT_ADDRESS)
                .typed(liveliness_proxy)
                .set_rewards_token_identifier(token_identifier)
                .run();
        }
    }

    pub fn set_per_block_rewards(
        &mut self,
        caller: TestAddress,
        per_block_amount: u64,
        expect: Option<ExpectError>,
    ) {
        if let Some(expect) = expect {
            self.world
                .tx()
                .from(caller)
                .to(LIVELINESS_STAKE_CONTRACT_ADDRESS)
                .typed(liveliness_proxy)
                .set_per_block_rewards(BigIntDec::from(per_block_amount, DEFAULT_DECIMALS))
                .with_result(expect)
                .run();
        } else {
            self.world
                .tx()
                .from(caller)
                .to(LIVELINESS_STAKE_CONTRACT_ADDRESS)
                .typed(liveliness_proxy)
                .set_per_block_rewards(BigIntDec::from(per_block_amount, DEFAULT_DECIMALS))
                .run();
        }
    }

    pub fn top_up_rewards(
        &mut self,
        caller: TestAddress,
        payment: u64,
        expect: Option<ExpectError>,
    ) {
        if let Some(expect) = expect {
            self.world
                .tx()
                .from(caller)
                .to(LIVELINESS_STAKE_CONTRACT_ADDRESS)
                .typed(liveliness_proxy)
                .top_up_rewards()
                .esdt(EsdtTokenPayment {
                    token_identifier: ITHEUM_TOKEN_IDENTIFIER.into(),
                    amount: BigIntDec::from(payment, DEFAULT_DECIMALS),
                    token_nonce: 0u64,
                })
                .with_result(expect)
                .run();
        } else {
            self.world
                .tx()
                .from(caller)
                .to(LIVELINESS_STAKE_CONTRACT_ADDRESS)
                .typed(liveliness_proxy)
                .top_up_rewards()
                .esdt(EsdtTokenPayment {
                    token_identifier: ITHEUM_TOKEN_IDENTIFIER.into(),
                    amount: BigIntDec::from(payment, DEFAULT_DECIMALS),
                    token_nonce: 0u64,
                })
                .run();
        }
    }

    pub fn withdraw_rewards(
        &mut self,
        caller: TestAddress,
        amount: u64,
        expect: Option<ExpectError>,
    ) {
        if let Some(expect) = expect {
            self.world
                .tx()
                .from(caller)
                .to(LIVELINESS_STAKE_CONTRACT_ADDRESS)
                .typed(liveliness_proxy)
                .withdraw_rewards(BigIntDec::from(amount, DEFAULT_DECIMALS))
                .with_result(expect)
                .run();
        } else {
            self.world
                .tx()
                .from(caller)
                .to(LIVELINESS_STAKE_CONTRACT_ADDRESS)
                .typed(liveliness_proxy)
                .withdraw_rewards(BigIntDec::from(amount, DEFAULT_DECIMALS))
                .run();
        }
    }

    pub fn claim_rewards(&mut self, caller: TestAddress, expect: Option<ExpectError>) {
        if let Some(expect) = expect {
            self.world
                .tx()
                .from(caller)
                .to(LIVELINESS_STAKE_CONTRACT_ADDRESS)
                .typed(liveliness_proxy)
                .claim_rewards()
                .with_result(expect)
                .run();
        } else {
            self.world
                .tx()
                .from(caller)
                .to(LIVELINESS_STAKE_CONTRACT_ADDRESS)
                .typed(liveliness_proxy)
                .claim_rewards()
                .run();
        }
    }
}
