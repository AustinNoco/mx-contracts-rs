// Code generated by the multiversx-sc build system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                            8
// Async Callback (empty):               1
// Total number of exported functions:  10

#![no_std]

multiversx_sc_wasm_adapter::allocator!();
multiversx_sc_wasm_adapter::panic_handler!();

multiversx_sc_wasm_adapter::external_view_init! {}

multiversx_sc_wasm_adapter::external_view_endpoints! {
    multisig
    (
        getPendingActionFullInfo => get_pending_action_full_info
        userRole => user_role
        getAllBoardMembers => get_all_board_members
        getAllProposers => get_all_proposers
        getActionData => get_action_data
        getActionSigners => get_action_signers
        getActionSignerCount => get_action_signer_count
        getActionValidSignerCount => get_action_valid_signer_count
    )
}

multiversx_sc_wasm_adapter::async_callback_empty! {}
