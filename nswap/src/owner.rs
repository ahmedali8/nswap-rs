//! Implement all the relevant logic for owner of this contract.

use crate::utils::FEE_DIVISOR;
use crate::*;

#[near_bindgen]
impl Contract {
    /// Change owner. Only can be called by owner.
    #[payable]
    pub fn set_owner(&mut self, owner_id: ValidAccountId) {
        assert_one_yocto();
        self.assert_owner();
        self.owner_id = owner_id.as_ref().clone();
    }

    /// Get the owner of this account.
    pub fn get_owner(&self) -> AccountId {
        self.owner_id.clone()
    }

    /// Extend guardians. Only can be called by owner.
    #[payable]
    pub fn extend_guardians(&mut self, guardians: Vec<ValidAccountId>) {
        assert_one_yocto();
        self.assert_owner();
        for guardian in guardians {
            self.guardians.insert(guardian.as_ref());
        }
    }

    /// Remove guardians. Only can be called by owner.
    #[payable]
    pub fn remove_guardians(&mut self, guardians: Vec<ValidAccountId>) {
        assert_one_yocto();
        self.assert_owner();
        for guardian in guardians {
            self.guardians.remove(guardian.as_ref());
        }
    }

    /// Change state of contract, Only can be called by owner or guardians.
    #[payable]
    pub fn change_state(&mut self, state: RunningState) {
        assert_one_yocto();
        assert!(self.is_owner_or_guardians(), "{}", ERR100_NOT_ALLOWED);

        if self.state != state {
            if state == RunningState::Running {
                // only owner can resume the contract
                self.assert_owner();
            }
            env::log(
                format!(
                    "Contract state changed from {} to {} by {}",
                    self.state,
                    state,
                    env::predecessor_account_id()
                )
                .as_bytes(),
            );
            self.state = state;
        }
    }

    /// Extend whitelisted tokens with new tokens. Only can be called by owner.
    #[payable]
    pub fn extend_whitelisted_tokens(&mut self, tokens: Vec<ValidAccountId>) {
        assert_one_yocto();
        assert!(self.is_owner_or_guardians(), "{}", ERR100_NOT_ALLOWED);
        for token in tokens {
            self.whitelisted_tokens.insert(token.as_ref());
        }
    }

    /// Remove whitelisted token. Only can be called by owner.
    #[payable]
    pub fn remove_whitelisted_tokens(&mut self, tokens: Vec<ValidAccountId>) {
        assert_one_yocto();
        assert!(self.is_owner_or_guardians(), "{}", ERR100_NOT_ALLOWED);
        for token in tokens {
            let exist = self.whitelisted_tokens.remove(token.as_ref());
            assert!(exist, "{}", ERR53_TOKEN_NOT_IN_LIST);
        }
    }

    /// Extend frozenlist tokens with new tokens.
    #[payable]
    pub fn extend_frozenlist_tokens(&mut self, tokens: Vec<ValidAccountId>) {
        assert_one_yocto();
        assert!(self.is_owner_or_guardians(), "{}", ERR100_NOT_ALLOWED);
        for token in tokens {
            self.frozen_tokens.insert(token.as_ref());
        }
    }

    /// Remove frozenlist token.
    #[payable]
    pub fn remove_frozenlist_tokens(&mut self, tokens: Vec<ValidAccountId>) {
        assert_one_yocto();
        assert!(self.is_owner_or_guardians(), "{}", ERR100_NOT_ALLOWED);
        for token in tokens {
            let exist = self.frozen_tokens.remove(token.as_ref());
            assert!(exist, "{}", ERR53_TOKEN_NOT_IN_LIST);
        }
    }

    #[payable]
    pub fn modify_admin_fee(&mut self, exchange_fee: u32, referral_fee: u32) {
        assert_one_yocto();
        self.assert_owner();
        assert!(
            exchange_fee + referral_fee <= FEE_DIVISOR,
            "{}",
            ERR101_ILLEGAL_FEE
        );
        self.exchange_fee = exchange_fee;
        self.referral_fee = referral_fee;
    }

    /// Remove exchange fee liquidity to owner's inner account.
    /// Owner's inner account storage should be prepared in advance.
    #[payable]
    pub fn remove_exchange_fee_liquidity(
        &mut self,
        pool_id: u64,
        shares: U128,
        min_amounts: Vec<U128>,
    ) {
        assert_one_yocto();
        assert!(self.is_owner_or_guardians(), "{}", ERR100_NOT_ALLOWED);
        self.assert_contract_running();
        let ex_id = env::current_account_id();
        let owner_id = self.owner_id.clone();
        let mut pool = self.pools.get(pool_id).expect(ERR85_NO_POOL);
        let amounts = pool.remove_liquidity(
            &ex_id,
            shares.into(),
            min_amounts.into_iter().map(|amount| amount.into()).collect(),
        );
        self.pools.replace(pool_id, &pool);
        let tokens = pool.tokens();
        let mut deposits = self.internal_unwrap_account(&owner_id);
        for i in 0..tokens.len() {
            deposits.deposit(&tokens[i], amounts[i]);
        }
        self.internal_save_account(&owner_id, deposits);
    }

    /// Withdraw owner inner account token to owner wallet.
    /// Owner inner account should be prepared in advance.
    #[payable]
    pub fn withdraw_owner_token(
        &mut self,
        token_id: ValidAccountId,
        amount: U128,
    ) -> Promise {
        assert_one_yocto();
        assert!(self.is_owner_or_guardians(), "{}", ERR100_NOT_ALLOWED);
        self.assert_contract_running();
        let token_id: AccountId = token_id.into();
        let amount: u128 = amount.into();
        assert!(amount > 0, "{}", ERR29_ILLEGAL_WITHDRAW_AMOUNT);
        let owner_id = self.owner_id.clone();
        let mut account = self.internal_unwrap_account(&owner_id);
        // Note: subtraction and deregistration will be reverted if the promise fails.
        account.withdraw(&token_id, amount);
        self.internal_save_account(&owner_id, account);
        self.internal_send_tokens(&owner_id, &token_id, amount)
    }

    pub(crate) fn assert_owner(&self) {
        assert_eq!(
            env::predecessor_account_id(),
            self.owner_id,
            "{}",
            ERR100_NOT_ALLOWED
        );
    }

    pub(crate) fn is_owner_or_guardians(&self) -> bool {
        env::predecessor_account_id() == self.owner_id
            || self.guardians.contains(&env::predecessor_account_id())
    }
}
