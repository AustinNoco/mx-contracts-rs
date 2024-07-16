// Code generated by the multiversx-sc proxy generator. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

#![allow(dead_code)]
#![allow(clippy::all)]

use multiversx_sc::proxy_imports::*;

pub struct PotlockProxy;

impl<Env, From, To, Gas> TxProxyTrait<Env, From, To, Gas> for PotlockProxy
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    type TxProxyMethods = PotlockProxyMethods<Env, From, To, Gas>;

    fn proxy_methods(self, tx: Tx<Env, From, To, (), Gas, (), ()>) -> Self::TxProxyMethods {
        PotlockProxyMethods { wrapped_tx: tx }
    }
}

pub struct PotlockProxyMethods<Env, From, To, Gas>
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    wrapped_tx: Tx<Env, From, To, (), Gas, (), ()>,
}

#[rustfmt::skip]
impl<Env, From, Gas> PotlockProxyMethods<Env, From, (), Gas>
where
    Env: TxEnv,
    Env::Api: VMApi,
    From: TxFrom<Env>,
    Gas: TxGas<Env>,
{
    pub fn init<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
    >(
        self,
        admin: Arg0,
    ) -> TxTypedDeploy<Env, From, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_deploy()
            .argument(&admin)
            .original_result()
    }
}

#[rustfmt::skip]
impl<Env, From, To, Gas> PotlockProxyMethods<Env, From, To, Gas>
where
    Env: TxEnv,
    Env::Api: VMApi,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    pub fn upgrade(
        self,
    ) -> TxTypedUpgrade<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_upgrade()
            .original_result()
    }
}

#[rustfmt::skip]
impl<Env, From, To, Gas> PotlockProxyMethods<Env, From, To, Gas>
where
    Env: TxEnv,
    Env::Api: VMApi,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    pub fn accept_pot<
        Arg0: ProxyArg<usize>,
    >(
        self,
        potlock_id: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("acceptPot")
            .argument(&potlock_id)
            .original_result()
    }

    pub fn remove_pot<
        Arg0: ProxyArg<usize>,
    >(
        self,
        potlock_id: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("removePot")
            .argument(&potlock_id)
            .original_result()
    }

    pub fn accept_application<
        Arg0: ProxyArg<usize>,
    >(
        self,
        project_id: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("acceptApplication")
            .argument(&project_id)
            .original_result()
    }

    pub fn reject_donation<
        Arg0: ProxyArg<usize>,
        Arg1: ProxyArg<ManagedAddress<Env::Api>>,
    >(
        self,
        potlock_id: Arg0,
        user: Arg1,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("rejectDonation")
            .argument(&potlock_id)
            .argument(&user)
            .original_result()
    }

    pub fn distribute_pot_to_projects<
        Arg0: ProxyArg<usize>,
        Arg1: ProxyArg<MultiValueEncoded<Env::Api, MultiValue2<usize, u64>>>,
    >(
        self,
        potlock_id: Arg0,
        project_percentage: Arg1,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("distributePotToProjects")
            .argument(&potlock_id)
            .argument(&project_percentage)
            .original_result()
    }

    pub fn add_pot<
        Arg0: ProxyArg<ManagedBuffer<Env::Api>>,
        Arg1: ProxyArg<ManagedBuffer<Env::Api>>,
    >(
        self,
        name: Arg0,
        description: Arg1,
    ) -> TxTypedCall<Env, From, To, (), Gas, ()> {
        self.wrapped_tx
            .raw_call("addPot")
            .argument(&name)
            .argument(&description)
            .original_result()
    }

    pub fn apply_for_pot<
        Arg0: ProxyArg<usize>,
        Arg1: ProxyArg<ManagedBuffer<Env::Api>>,
        Arg2: ProxyArg<ManagedBuffer<Env::Api>>,
    >(
        self,
        potlock_id: Arg0,
        project_name: Arg1,
        description: Arg2,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("applyForPot")
            .argument(&potlock_id)
            .argument(&project_name)
            .argument(&description)
            .original_result()
    }

    pub fn donate_to_pot<
        Arg0: ProxyArg<usize>,
    >(
        self,
        potlock_id: Arg0,
    ) -> TxTypedCall<Env, From, To, (), Gas, ()> {
        self.wrapped_tx
            .raw_call("donateToPot")
            .argument(&potlock_id)
            .original_result()
    }

    pub fn donate_to_project<
        Arg0: ProxyArg<usize>,
    >(
        self,
        project_id: Arg0,
    ) -> TxTypedCall<Env, From, To, (), Gas, ()> {
        self.wrapped_tx
            .raw_call("donateToProject")
            .argument(&project_id)
            .original_result()
    }

    pub fn change_fee_for_pots<
        Arg0: ProxyArg<TokenIdentifier<Env::Api>>,
        Arg1: ProxyArg<BigUint<Env::Api>>,
    >(
        self,
        token_identifier: Arg0,
        fee: Arg1,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("changeFeeForPots")
            .argument(&token_identifier)
            .argument(&fee)
            .original_result()
    }

    pub fn fee_token_identifier(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, TokenIdentifier<Env::Api>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getFeeTokenIdentifier")
            .original_result()
    }

    pub fn fee_amount(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, BigUint<Env::Api>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getFeeAmount")
            .original_result()
    }

    pub fn potlocks(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, MultiValueEncoded<Env::Api, Pot<Env::Api>>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getPotlocks")
            .original_result()
    }

    pub fn projects(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, MultiValueEncoded<Env::Api, Project<Env::Api>>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getProjects")
            .original_result()
    }

    pub fn fee_pot_proposer<
        Arg0: ProxyArg<usize>,
    >(
        self,
        potlock_id: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ManagedAddress<Env::Api>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("feePotPayments")
            .argument(&potlock_id)
            .original_result()
    }

    pub fn fee_amount_accepted_pots(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, BigUint<Env::Api>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("feeAmountAcceptPots")
            .original_result()
    }

    pub fn pot_donations<
        Arg0: ProxyArg<usize>,
    >(
        self,
        project_id: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, MultiValueEncoded<Env::Api, MultiValue2<ManagedAddress<Env::Api>, EsdtTokenPayment<Env::Api>>>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("potDonations")
            .argument(&project_id)
            .original_result()
    }

    pub fn project_donations<
        Arg0: ProxyArg<usize>,
    >(
        self,
        project_id: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, MultiValueEncoded<Env::Api, MultiValue2<ManagedAddress<Env::Api>, EsdtTokenPayment<Env::Api>>>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("projectDonations")
            .argument(&project_id)
            .original_result()
    }

    pub fn is_admin<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
    >(
        self,
        address: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, bool> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("isAdmin")
            .argument(&address)
            .original_result()
    }

    pub fn add_admin<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
    >(
        self,
        address: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("addAdmin")
            .argument(&address)
            .original_result()
    }

    pub fn remove_admin<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
    >(
        self,
        address: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("removeAdmin")
            .argument(&address)
            .original_result()
    }

    pub fn admins(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, MultiValueEncoded<Env::Api, ManagedAddress<Env::Api>>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getAdmins")
            .original_result()
    }
}

#[type_abi]
#[derive(TopEncode, TopDecode)]
pub struct Pot<Api>
where
    Api: ManagedTypeApi,
{
    pub potlock_id: usize,
    pub token_identifier: TokenIdentifier<Api>,
    pub fee: BigUint<Api>,
    pub name: ManagedBuffer<Api>,
    pub description: ManagedBuffer<Api>,
    pub status: Status,
}

#[type_abi]
#[derive(TopEncode, TopDecode, PartialEq, Eq, Debug, NestedEncode, NestedDecode)]
pub enum Status {
    Inactive,
    Active,
}

#[type_abi]
#[derive(TopEncode, TopDecode)]
pub struct Project<Api>
where
    Api: ManagedTypeApi,
{
    pub project_id: usize,
    pub potlock_id: usize,
    pub name: ManagedBuffer<Api>,
    pub description: ManagedBuffer<Api>,
    pub owner: ManagedAddress<Api>,
    pub status: Status,
}
