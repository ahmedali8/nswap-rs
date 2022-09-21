// Storage errors.

pub const ERR10_ACC_NOT_REGISTERED: &str = "E10: account not registered";
pub const ERR11_INSUFFICIENT_STORAGE: &str =
    "E11: insufficient $NEAR storage deposit";
pub const ERR12_TOKEN_NOT_WHITELISTED: &str = "E12: token not whitelisted";
pub const ERR13_LP_NOT_REGISTERED: &str = "E13: LP not registered";
pub const ERR14_LP_ALREADY_REGISTERED: &str = "E14: LP already registered";
pub const ERR15_NO_STORAGE_CAN_WITHDRAW: &str = "E15: no storage can withdraw";
pub const ERR16_STORAGE_WITHDRAW_TOO_MUCH: &str =
    "E16: storage withdraw too much";
pub const ERR17_DEPOSIT_LESS_THAN_MIN_STORAGE: &str =
    "E17: deposit less than min storage";
pub const ERR18_TOKENS_NOT_EMPTY: &str =
    "E18: storage unregister tokens not empty";

// Accounts.

pub const ERR21_TOKEN_NOT_REG: &str = "E21: token not registered";
pub const ERR22_NOT_ENOUGH_TOKENS: &str = "E22: not enough tokens in deposit";
// pub const ERR23_NOT_ENOUGH_NEAR: &str = "E23: not enough NEAR in deposit";
pub const ERR24_NON_ZERO_TOKEN_BALANCE: &str = "E24: non-zero token balance";
pub const ERR25_CALLBACK_POST_WITHDRAW_INVALID: &str =
    "E25: expected 1 promise result from withdraw";
// pub const ERR26_ACCESS_KEY_NOT_ALLOWED: &str = "E26: access key not allowed";
pub const ERR27_DEPOSIT_NEEDED: &str =
    "E27: attach 1yN to swap tokens not in whitelist";
pub const ERR28_WRONG_MSG_FORMAT: &str = "E28: Illegal msg in ft_transfer_call";
pub const ERR29_ILLEGAL_WITHDRAW_AMOUNT: &str = "E29: Illegal withdraw amount";

// Liquidity operations.

pub const ERR31_ZERO_AMOUNT: &str = "E31: adding zero amount";
pub const ERR32_ZERO_SHARES: &str = "E32: minting zero shares";
pub const ERR33_TRANSFER_TO_SELF: &str = "E33: transfer to self";

pub const ERR35_AT_LEAST_ONE_YOCTO: &str =
    "E35: requires attached deposit of at least 1 yoctoNEAR";
pub const ERR36_SHARES_TOTAL_SUPPLY_OVERFLOW: &str =
    "E36: shares_total_supply overflow";

// Action result.

pub const ERR41_WRONG_ACTION_RESULT: &str = "E41: wrong action result type";

// Contract Level
pub const ERR51_CONTRACT_PAUSED: &str = "E51: contract paused";
pub const ERR52_FROZEN_TOKEN: &str = "E52: token frozen";
pub const ERR53_TOKEN_NOT_IN_LIST: &str = "E53: token not in list";

// Swap
pub const ERR68_SLIPPAGE: &str = "E68: slippage error";
pub const ERR72_AT_LEAST_ONE_SWAP: &str = "E72: at least one swap";
pub const ERR73_SAME_TOKEN: &str = "E73: same token swap";
pub const ERR75_INVARIANT_REDUCE: &str = "E75: invariant can not reduce ";
pub const ERR76_INVALID_PARAMS: &str = "E76: invalid params";

// pool manage
pub const ERR85_NO_POOL: &str = "E85: invalid pool id";
pub const ERR86_MIN_AMOUNT: &str = "E86: amount need above min amount";
pub const ERR87_ILLEGAL_POOL_ID: &str = "E87: illegal pool id";
pub const ERR89_WRONG_TOKEN_COUNT: &str = "E89: wrong token count";
pub const ERR90_FEE_TOO_LARGE: &str = "E90: fee too large";
pub const ERR91_NOT_ENOUGH_SHARES: &str = "E91: not enough shares";
pub const ERR92_TOKEN_DUPLICATES: &str = "E92: token duplicated";
pub const ERR89_WRONG_AMOUNT_COUNT: &str = "E89: wrong amount count";

// owner
pub const ERR100_NOT_ALLOWED: &str = "E100: no permission to invoke this";
pub const ERR101_ILLEGAL_FEE: &str = "E101: illegal fee";
pub const ERR102_INVALID_TOKEN_ID: &str = "E102: invalid token id";
pub const ERR103_NOT_INITIALIZED: &str = "E103: contract is not initialized";

//mft
pub const ERR110_INVALID_REGISTER: &str = "E110: Invalid register";
