// Code generated by the multiversx-sc build system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Upgrade:                              1
// Endpoints:                            5
// Async Callback (empty):               1
// Total number of exported functions:   8

#![no_std]

multiversx_sc_wasm_adapter::allocator!();
multiversx_sc_wasm_adapter::panic_handler!();

multiversx_sc_wasm_adapter::endpoints! {
    core_mx_liveliness_stake
    (
        init => init
        upgrade => upgrade
        setContractStateActive => set_contract_state_active
        setContractStateInactive => set_contract_state_inactive
        setAdministrator => set_administrator
        getContractState => contract_state
        getAdministrator => administrator
    )
}

multiversx_sc_wasm_adapter::async_callback_empty! {}
