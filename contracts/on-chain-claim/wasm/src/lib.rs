// Code generated by the multiversx-sc build system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                            6
// Async Callback (empty):               1
// Total number of exported functions:   8

#![no_std]

// Configuration that works with rustc < 1.73.0.
// TODO: Recommended rustc version: 1.73.0 or newer.
#![feature(lang_items)]

multiversx_sc_wasm_adapter::allocator!();
multiversx_sc_wasm_adapter::panic_handler!();

multiversx_sc_wasm_adapter::endpoints! {
    on_chain_claim
    (
        init => init
        claim => claim
        claimAndRepair => claim_and_repair
        updateState => update_state
        canBeRepaired => can_be_repaired
        getAddressInfo => address_info
        getRepairStreakTokenIdentifier => repair_streak_token_identifier
    )
}

multiversx_sc_wasm_adapter::async_callback_empty! {}
