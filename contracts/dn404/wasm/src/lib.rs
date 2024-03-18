// Code generated by the multiversx-sc build system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                           22
// Async Callback:                       1
// Total number of exported functions:  24

#![no_std]
#![allow(internal_features)]
#![feature(lang_items)]

multiversx_sc_wasm_adapter::allocator!();
multiversx_sc_wasm_adapter::panic_handler!();

multiversx_sc_wasm_adapter::endpoints! {
    dn404
    (
        init => init
        upgrade => upgrade
        deposit => deposit
        depositBasketOfGoods => deposit_basket_of_goods
        claimBasketOfGoods => claim_basket_of_goods
        getBasketOfGoods => basket_of_goods
        getRemainingTokens => remaining_tokens
        setInternalPriceForToken => set_internal_price_for_token
        setInternalPriceForCollection => set_internal_price_for_collection
        getPriceForToken => try_get_price
        getFractalTokenId => fractal_token
        setFeeForFractionalisingNft => set_fee_for_fractionalizing_nft
        setFeeForFactionalisingCollection => set_fee_for_fractionalizing_collection
        setFeeForDepositBaskedOfGoods => set_fee_for_deposit_basket_of_goods
        getFee => get_fee
        getFeePercentageForBasketDeposit => fee_basket
        pause => pause_endpoint
        unpause => unpause_endpoint
        isPaused => paused_status
        isAdmin => is_admin
        addAdmin => add_admin
        removeAdmin => remove_admin
        getAdmins => admins
    )
}

multiversx_sc_wasm_adapter::async_callback! { dn404 }