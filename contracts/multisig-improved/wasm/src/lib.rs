// Code generated by the multiversx-sc build system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                           40
// Async Callback:                       1
// Total number of exported functions:  42

#![no_std]
#![allow(internal_features)]
#![feature(lang_items)]

multiversx_sc_wasm_adapter::allocator!();
multiversx_sc_wasm_adapter::panic_handler!();

multiversx_sc_wasm_adapter::endpoints! {
    multisig_improved
    (
        init => init
        upgrade => upgrade
        deposit => deposit
        getQuorum => quorum
        getNumBoardMembers => num_board_members
        getNumGroups => num_groups
        getNumProposers => num_proposers
        getActionGroup => action_groups
        getLastGroupActionId => last_action_group_id
        quorumReached => quorum_reached
        proposeAddBoardMember => propose_add_board_member
        proposeAddProposer => propose_add_proposer
        proposeRemoveUser => propose_remove_user
        proposeChangeQuorum => propose_change_quorum
        proposeTransferExecute => propose_transfer_execute
        proposeTransferExecuteEsdt => propose_transfer_execute_esdt
        proposeAsyncCall => propose_async_call
        proposeSCDeployFromSource => propose_sc_deploy_from_source
        proposeSCUpgradeFromSource => propose_sc_upgrade_from_source
        proposeModuleDeployFromSource => propose_module_deploy_from_source
        proposeModuleUpgradeFromSource => propose_module_upgrade_from_source
        proposeBatch => propose_batch
        performAction => perform_action_endpoint
        performBatch => perform_batch
        discardAction => discard_action_endpoint
        discardBatch => discard_batch
        sign => sign
        signBatch => sign_batch
        signAndPerform => sign_and_perform
        signBatchAndPerform => sign_batch_and_perform
        unsign => unsign
        unsignBatch => unsign_batch
        unsignForOutdatedBoardMembers => unsign_for_outdated_board_members
        enableModule => enable_module
        disableModule => disable_module
        addAdditionalAllowedAddresses => add_additional_allowed_addresses
        removeAdditionalAllowedAddresses => remove_additional_allowed_addresses
        signed => signed
        getActionLastIndex => get_action_last_index
        getUserNonce => get_user_nonce
        dnsRegister => dns_register
    )
}

multiversx_sc_wasm_adapter::async_callback! { multisig_improved }
