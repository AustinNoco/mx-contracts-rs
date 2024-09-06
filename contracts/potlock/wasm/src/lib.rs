// Code generated by the multiversx-sc build system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Upgrade:                              1
// Endpoints:                           21
// Async Callback (empty):               1
// Total number of exported functions:  24

#![no_std]

multiversx_sc_wasm_adapter::allocator!();
multiversx_sc_wasm_adapter::panic_handler!();

multiversx_sc_wasm_adapter::endpoints! {
    potlock
    (
        init => init
        upgrade => upgrade
        changeFeeForPots => change_fee_for_pots
        acceptPot => accept_pot
        removePot => remove_pot
        acceptApplication => accept_application
        removeApplication => remove_application
        rejectDonation => reject_donation
        distributePotToProjects => distribute_pot_to_projects
        addPot => add_pot
        applyForPot => apply_for_pot
        donateToPot => donate_to_pot
        donateToProject => donate_to_project
        getFeeTokenIdentifier => fee_token_identifier
        getFeeAmount => fee_amount
        getPotlocks => potlocks
        getProjects => projects
        potDonations => pot_donations
        projectDonations => project_donations
        isAdmin => is_admin
        addAdmin => add_admin
        removeAdmin => remove_admin
        getAdmins => admins
    )
}

multiversx_sc_wasm_adapter::async_callback_empty! {}