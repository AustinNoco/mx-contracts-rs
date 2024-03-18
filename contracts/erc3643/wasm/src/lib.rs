// Code generated by the multiversx-sc build system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                           12
// Async Callback:                       1
// Total number of exported functions:  14

#![no_std]
#![allow(internal_features)]
#![feature(lang_items)]

multiversx_sc_wasm_adapter::allocator!();
multiversx_sc_wasm_adapter::panic_handler!();

multiversx_sc_wasm_adapter::endpoints! {
    erc3643
    (
        init => init
        upgrade => upgrade
        addUsers => add_users
        removeUsers => remove_users
        isUserWhitelisted => is_user_whitelisted
        registerToken => register_token
        setTransferRole => set_transfer_role
        getTokenId => token
        addHook => add_hook
        removeHook => remove_hook
        pause => pause_endpoint
        unpause => unpause_endpoint
        isPaused => paused_status
    )
}

multiversx_sc_wasm_adapter::async_callback! { erc3643 }