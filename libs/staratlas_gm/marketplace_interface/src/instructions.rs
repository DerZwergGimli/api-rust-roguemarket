use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult,
    instruction::{AccountMeta, Instruction},
    program::{invoke, invoke_signed},
    pubkey::Pubkey,
};

pub const ADD_ROYALTY_TIER_IX_ACCOUNTS_LEN: usize = 3usize;

#[derive(Copy, Clone, Debug)]
pub struct AddRoyaltyTierAccounts<'me, 'a0: 'me, 'a1: 'me, 'a2: 'me> {
    pub update_authority_account: &'me AccountInfo<'a0>,
    pub market_vars_account: &'me AccountInfo<'a1>,
    pub registered_currency: &'me AccountInfo<'a2>,
}

#[derive(Copy, Clone, Debug)]
pub struct AddRoyaltyTierKeys {
    pub update_authority_account: Pubkey,
    pub market_vars_account: Pubkey,
    pub registered_currency: Pubkey,
}

impl<'me> From<&AddRoyaltyTierAccounts<'me, '_, '_, '_>> for AddRoyaltyTierKeys {
    fn from(accounts: &AddRoyaltyTierAccounts<'me, '_, '_, '_>) -> Self {
        Self {
            update_authority_account: *accounts.update_authority_account.key,
            market_vars_account: *accounts.market_vars_account.key,
            registered_currency: *accounts.registered_currency.key,
        }
    }
}

impl From<&AddRoyaltyTierKeys> for [AccountMeta; ADD_ROYALTY_TIER_IX_ACCOUNTS_LEN] {
    fn from(keys: &AddRoyaltyTierKeys) -> Self {
        [
            AccountMeta::new(keys.update_authority_account, true),
            AccountMeta::new_readonly(keys.market_vars_account, false),
            AccountMeta::new(keys.registered_currency, false),
        ]
    }
}

impl<'a> From<&AddRoyaltyTierAccounts<'_, 'a, 'a, 'a>>
for [AccountInfo<'a>; ADD_ROYALTY_TIER_IX_ACCOUNTS_LEN] {
    fn from(accounts: &AddRoyaltyTierAccounts<'_, 'a, 'a, 'a>) -> Self {
        [
            accounts.update_authority_account.clone(),
            accounts.market_vars_account.clone(),
            accounts.registered_currency.clone(),
        ]
    }
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct AddRoyaltyTierIxArgs {
    pub stake_amount: u64,
    pub discount: u64,
}

#[derive(Copy, Clone, Debug)]
pub struct AddRoyaltyTierIxData<'me>(pub &'me AddRoyaltyTierIxArgs);

pub const ADD_ROYALTY_TIER_IX_DISCM: [u8; 8] = [233, 33, 85, 96, 142, 116, 240, 66];

impl<'me> From<&'me AddRoyaltyTierIxArgs> for AddRoyaltyTierIxData<'me> {
    fn from(args: &'me AddRoyaltyTierIxArgs) -> Self {
        Self(args)
    }
}

impl BorshSerialize for AddRoyaltyTierIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&ADD_ROYALTY_TIER_IX_DISCM)?;
        self.0.serialize(writer)
    }
}

pub fn add_royalty_tier_ix<K: Into<AddRoyaltyTierKeys>, A: Into<AddRoyaltyTierIxArgs>>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: AddRoyaltyTierKeys = accounts.into();
    let metas: [AccountMeta; ADD_ROYALTY_TIER_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: AddRoyaltyTierIxArgs = args.into();
    let data: AddRoyaltyTierIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}

pub fn add_royalty_tier_invoke<'a, A: Into<AddRoyaltyTierIxArgs>>(
    accounts: &AddRoyaltyTierAccounts<'_, 'a, 'a, 'a>,
    args: A,
) -> ProgramResult {
    let ix = add_royalty_tier_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; ADD_ROYALTY_TIER_IX_ACCOUNTS_LEN] = accounts
        .into();
    invoke(&ix, &account_info)
}

pub fn add_royalty_tier_invoke_signed<'a, A: Into<AddRoyaltyTierIxArgs>>(
    accounts: &AddRoyaltyTierAccounts<'_, 'a, 'a, 'a>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = add_royalty_tier_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; ADD_ROYALTY_TIER_IX_ACCOUNTS_LEN] = accounts
        .into();
    invoke_signed(&ix, &account_info, seeds)
}

pub const DELETE_ROYALTY_TIER_IX_ACCOUNTS_LEN: usize = 3usize;

#[derive(Copy, Clone, Debug)]
pub struct DeleteRoyaltyTierAccounts<'me, 'a0: 'me, 'a1: 'me, 'a2: 'me> {
    pub update_authority_account: &'me AccountInfo<'a0>,
    pub market_vars_account: &'me AccountInfo<'a1>,
    pub registered_currency: &'me AccountInfo<'a2>,
}

#[derive(Copy, Clone, Debug)]
pub struct DeleteRoyaltyTierKeys {
    pub update_authority_account: Pubkey,
    pub market_vars_account: Pubkey,
    pub registered_currency: Pubkey,
}

impl<'me> From<&DeleteRoyaltyTierAccounts<'me, '_, '_, '_>> for DeleteRoyaltyTierKeys {
    fn from(accounts: &DeleteRoyaltyTierAccounts<'me, '_, '_, '_>) -> Self {
        Self {
            update_authority_account: *accounts.update_authority_account.key,
            market_vars_account: *accounts.market_vars_account.key,
            registered_currency: *accounts.registered_currency.key,
        }
    }
}

impl From<&DeleteRoyaltyTierKeys>
for [AccountMeta; DELETE_ROYALTY_TIER_IX_ACCOUNTS_LEN] {
    fn from(keys: &DeleteRoyaltyTierKeys) -> Self {
        [
            AccountMeta::new(keys.update_authority_account, true),
            AccountMeta::new_readonly(keys.market_vars_account, false),
            AccountMeta::new(keys.registered_currency, false),
        ]
    }
}

impl<'a> From<&DeleteRoyaltyTierAccounts<'_, 'a, 'a, 'a>>
for [AccountInfo<'a>; DELETE_ROYALTY_TIER_IX_ACCOUNTS_LEN] {
    fn from(accounts: &DeleteRoyaltyTierAccounts<'_, 'a, 'a, 'a>) -> Self {
        [
            accounts.update_authority_account.clone(),
            accounts.market_vars_account.clone(),
            accounts.registered_currency.clone(),
        ]
    }
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct DeleteRoyaltyTierIxArgs {
    pub stake_amount: u64,
}

#[derive(Copy, Clone, Debug)]
pub struct DeleteRoyaltyTierIxData<'me>(pub &'me DeleteRoyaltyTierIxArgs);

pub const DELETE_ROYALTY_TIER_IX_DISCM: [u8; 8] = [74, 81, 94, 157, 102, 156, 188, 109];

impl<'me> From<&'me DeleteRoyaltyTierIxArgs> for DeleteRoyaltyTierIxData<'me> {
    fn from(args: &'me DeleteRoyaltyTierIxArgs) -> Self {
        Self(args)
    }
}

impl BorshSerialize for DeleteRoyaltyTierIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&DELETE_ROYALTY_TIER_IX_DISCM)?;
        self.0.serialize(writer)
    }
}

pub fn delete_royalty_tier_ix<
    K: Into<DeleteRoyaltyTierKeys>,
    A: Into<DeleteRoyaltyTierIxArgs>,
>(accounts: K, args: A) -> std::io::Result<Instruction> {
    let keys: DeleteRoyaltyTierKeys = accounts.into();
    let metas: [AccountMeta; DELETE_ROYALTY_TIER_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: DeleteRoyaltyTierIxArgs = args.into();
    let data: DeleteRoyaltyTierIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}

pub fn delete_royalty_tier_invoke<'a, A: Into<DeleteRoyaltyTierIxArgs>>(
    accounts: &DeleteRoyaltyTierAccounts<'_, 'a, 'a, 'a>,
    args: A,
) -> ProgramResult {
    let ix = delete_royalty_tier_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; DELETE_ROYALTY_TIER_IX_ACCOUNTS_LEN] = accounts
        .into();
    invoke(&ix, &account_info)
}

pub fn delete_royalty_tier_invoke_signed<'a, A: Into<DeleteRoyaltyTierIxArgs>>(
    accounts: &DeleteRoyaltyTierAccounts<'_, 'a, 'a, 'a>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = delete_royalty_tier_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; DELETE_ROYALTY_TIER_IX_ACCOUNTS_LEN] = accounts
        .into();
    invoke_signed(&ix, &account_info, seeds)
}

pub const DEREGISTER_CURRENCY_IX_ACCOUNTS_LEN: usize = 5usize;

#[derive(Copy, Clone, Debug)]
pub struct DeregisterCurrencyAccounts<
    'me,
    'a0: 'me,
    'a1: 'me,
    'a2: 'me,
    'a3: 'me,
    'a4: 'me,
> {
    pub update_authority_account: &'me AccountInfo<'a0>,
    pub market_vars_account: &'me AccountInfo<'a1>,
    pub registered_currency: &'me AccountInfo<'a2>,
    pub currency_mint: &'me AccountInfo<'a3>,
    pub system_program: &'me AccountInfo<'a4>,
}

#[derive(Copy, Clone, Debug)]
pub struct DeregisterCurrencyKeys {
    pub update_authority_account: Pubkey,
    pub market_vars_account: Pubkey,
    pub registered_currency: Pubkey,
    pub currency_mint: Pubkey,
    pub system_program: Pubkey,
}

impl<'me> From<&DeregisterCurrencyAccounts<'me, '_, '_, '_, '_, '_>>
for DeregisterCurrencyKeys {
    fn from(accounts: &DeregisterCurrencyAccounts<'me, '_, '_, '_, '_, '_>) -> Self {
        Self {
            update_authority_account: *accounts.update_authority_account.key,
            market_vars_account: *accounts.market_vars_account.key,
            registered_currency: *accounts.registered_currency.key,
            currency_mint: *accounts.currency_mint.key,
            system_program: *accounts.system_program.key,
        }
    }
}

impl From<&DeregisterCurrencyKeys>
for [AccountMeta; DEREGISTER_CURRENCY_IX_ACCOUNTS_LEN] {
    fn from(keys: &DeregisterCurrencyKeys) -> Self {
        [
            AccountMeta::new(keys.update_authority_account, true),
            AccountMeta::new_readonly(keys.market_vars_account, false),
            AccountMeta::new(keys.registered_currency, false),
            AccountMeta::new_readonly(keys.currency_mint, false),
            AccountMeta::new_readonly(keys.system_program, false),
        ]
    }
}

impl<'a> From<&DeregisterCurrencyAccounts<'_, 'a, 'a, 'a, 'a, 'a>>
for [AccountInfo<'a>; DEREGISTER_CURRENCY_IX_ACCOUNTS_LEN] {
    fn from(accounts: &DeregisterCurrencyAccounts<'_, 'a, 'a, 'a, 'a, 'a>) -> Self {
        [
            accounts.update_authority_account.clone(),
            accounts.market_vars_account.clone(),
            accounts.registered_currency.clone(),
            accounts.currency_mint.clone(),
            accounts.system_program.clone(),
        ]
    }
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct DeregisterCurrencyIxArgs {}

#[derive(Copy, Clone, Debug)]
pub struct DeregisterCurrencyIxData<'me>(pub &'me DeregisterCurrencyIxArgs);

pub const DEREGISTER_CURRENCY_IX_DISCM: [u8; 8] = [189, 233, 33, 25, 55, 216, 28, 90];

impl<'me> From<&'me DeregisterCurrencyIxArgs> for DeregisterCurrencyIxData<'me> {
    fn from(args: &'me DeregisterCurrencyIxArgs) -> Self {
        Self(args)
    }
}

impl BorshSerialize for DeregisterCurrencyIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&DEREGISTER_CURRENCY_IX_DISCM)?;
        self.0.serialize(writer)
    }
}

pub fn deregister_currency_ix<
    K: Into<DeregisterCurrencyKeys>,
    A: Into<DeregisterCurrencyIxArgs>,
>(accounts: K, args: A) -> std::io::Result<Instruction> {
    let keys: DeregisterCurrencyKeys = accounts.into();
    let metas: [AccountMeta; DEREGISTER_CURRENCY_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: DeregisterCurrencyIxArgs = args.into();
    let data: DeregisterCurrencyIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}

pub fn deregister_currency_invoke<'a, A: Into<DeregisterCurrencyIxArgs>>(
    accounts: &DeregisterCurrencyAccounts<'_, 'a, 'a, 'a, 'a, 'a>,
    args: A,
) -> ProgramResult {
    let ix = deregister_currency_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; DEREGISTER_CURRENCY_IX_ACCOUNTS_LEN] = accounts
        .into();
    invoke(&ix, &account_info)
}

pub fn deregister_currency_invoke_signed<'a, A: Into<DeregisterCurrencyIxArgs>>(
    accounts: &DeregisterCurrencyAccounts<'_, 'a, 'a, 'a, 'a, 'a>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = deregister_currency_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; DEREGISTER_CURRENCY_IX_ACCOUNTS_LEN] = accounts
        .into();
    invoke_signed(&ix, &account_info, seeds)
}

pub const INITIALIZE_MARKETPLACE_IX_ACCOUNTS_LEN: usize = 3usize;

#[derive(Copy, Clone, Debug)]
pub struct InitializeMarketplaceAccounts<'me, 'a0: 'me, 'a1: 'me, 'a2: 'me> {
    pub update_authority_account: &'me AccountInfo<'a0>,
    pub market_vars_account: &'me AccountInfo<'a1>,
    pub system_program: &'me AccountInfo<'a2>,
}

#[derive(Copy, Clone, Debug)]
pub struct InitializeMarketplaceKeys {
    pub update_authority_account: Pubkey,
    pub market_vars_account: Pubkey,
    pub system_program: Pubkey,
}

impl<'me> From<&InitializeMarketplaceAccounts<'me, '_, '_, '_>>
for InitializeMarketplaceKeys {
    fn from(accounts: &InitializeMarketplaceAccounts<'me, '_, '_, '_>) -> Self {
        Self {
            update_authority_account: *accounts.update_authority_account.key,
            market_vars_account: *accounts.market_vars_account.key,
            system_program: *accounts.system_program.key,
        }
    }
}

impl From<&InitializeMarketplaceKeys>
for [AccountMeta; INITIALIZE_MARKETPLACE_IX_ACCOUNTS_LEN] {
    fn from(keys: &InitializeMarketplaceKeys) -> Self {
        [
            AccountMeta::new(keys.update_authority_account, true),
            AccountMeta::new(keys.market_vars_account, false),
            AccountMeta::new_readonly(keys.system_program, false),
        ]
    }
}

impl<'a> From<&InitializeMarketplaceAccounts<'_, 'a, 'a, 'a>>
for [AccountInfo<'a>; INITIALIZE_MARKETPLACE_IX_ACCOUNTS_LEN] {
    fn from(accounts: &InitializeMarketplaceAccounts<'_, 'a, 'a, 'a>) -> Self {
        [
            accounts.update_authority_account.clone(),
            accounts.market_vars_account.clone(),
            accounts.system_program.clone(),
        ]
    }
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct InitializeMarketplaceIxArgs {}

#[derive(Copy, Clone, Debug)]
pub struct InitializeMarketplaceIxData<'me>(pub &'me InitializeMarketplaceIxArgs);

pub const INITIALIZE_MARKETPLACE_IX_DISCM: [u8; 8] = [47, 81, 64, 0, 96, 56, 105, 7];

impl<'me> From<&'me InitializeMarketplaceIxArgs> for InitializeMarketplaceIxData<'me> {
    fn from(args: &'me InitializeMarketplaceIxArgs) -> Self {
        Self(args)
    }
}

impl BorshSerialize for InitializeMarketplaceIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_MARKETPLACE_IX_DISCM)?;
        self.0.serialize(writer)
    }
}

pub fn initialize_marketplace_ix<
    K: Into<InitializeMarketplaceKeys>,
    A: Into<InitializeMarketplaceIxArgs>,
>(accounts: K, args: A) -> std::io::Result<Instruction> {
    let keys: InitializeMarketplaceKeys = accounts.into();
    let metas: [AccountMeta; INITIALIZE_MARKETPLACE_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: InitializeMarketplaceIxArgs = args.into();
    let data: InitializeMarketplaceIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}

pub fn initialize_marketplace_invoke<'a, A: Into<InitializeMarketplaceIxArgs>>(
    accounts: &InitializeMarketplaceAccounts<'_, 'a, 'a, 'a>,
    args: A,
) -> ProgramResult {
    let ix = initialize_marketplace_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; INITIALIZE_MARKETPLACE_IX_ACCOUNTS_LEN] = accounts
        .into();
    invoke(&ix, &account_info)
}

pub fn initialize_marketplace_invoke_signed<'a, A: Into<InitializeMarketplaceIxArgs>>(
    accounts: &InitializeMarketplaceAccounts<'_, 'a, 'a, 'a>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = initialize_marketplace_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; INITIALIZE_MARKETPLACE_IX_ACCOUNTS_LEN] = accounts
        .into();
    invoke_signed(&ix, &account_info, seeds)
}

pub const REGISTER_CURRENCY_IX_ACCOUNTS_LEN: usize = 6usize;

#[derive(Copy, Clone, Debug)]
pub struct RegisterCurrencyAccounts<
    'me,
    'a0: 'me,
    'a1: 'me,
    'a2: 'me,
    'a3: 'me,
    'a4: 'me,
    'a5: 'me,
> {
    pub update_authority_account: &'me AccountInfo<'a0>,
    pub market_vars_account: &'me AccountInfo<'a1>,
    pub registered_currency: &'me AccountInfo<'a2>,
    pub currency_mint: &'me AccountInfo<'a3>,
    pub sa_currency_vault: &'me AccountInfo<'a4>,
    pub system_program: &'me AccountInfo<'a5>,
}

#[derive(Copy, Clone, Debug)]
pub struct RegisterCurrencyKeys {
    pub update_authority_account: Pubkey,
    pub market_vars_account: Pubkey,
    pub registered_currency: Pubkey,
    pub currency_mint: Pubkey,
    pub sa_currency_vault: Pubkey,
    pub system_program: Pubkey,
}

impl<'me> From<&RegisterCurrencyAccounts<'me, '_, '_, '_, '_, '_, '_>>
for RegisterCurrencyKeys {
    fn from(accounts: &RegisterCurrencyAccounts<'me, '_, '_, '_, '_, '_, '_>) -> Self {
        Self {
            update_authority_account: *accounts.update_authority_account.key,
            market_vars_account: *accounts.market_vars_account.key,
            registered_currency: *accounts.registered_currency.key,
            currency_mint: *accounts.currency_mint.key,
            sa_currency_vault: *accounts.sa_currency_vault.key,
            system_program: *accounts.system_program.key,
        }
    }
}

impl From<&RegisterCurrencyKeys> for [AccountMeta; REGISTER_CURRENCY_IX_ACCOUNTS_LEN] {
    fn from(keys: &RegisterCurrencyKeys) -> Self {
        [
            AccountMeta::new(keys.update_authority_account, true),
            AccountMeta::new_readonly(keys.market_vars_account, false),
            AccountMeta::new(keys.registered_currency, false),
            AccountMeta::new_readonly(keys.currency_mint, false),
            AccountMeta::new_readonly(keys.sa_currency_vault, false),
            AccountMeta::new_readonly(keys.system_program, false),
        ]
    }
}

impl<'a> From<&RegisterCurrencyAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a>>
for [AccountInfo<'a>; REGISTER_CURRENCY_IX_ACCOUNTS_LEN] {
    fn from(accounts: &RegisterCurrencyAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a>) -> Self {
        [
            accounts.update_authority_account.clone(),
            accounts.market_vars_account.clone(),
            accounts.registered_currency.clone(),
            accounts.currency_mint.clone(),
            accounts.sa_currency_vault.clone(),
            accounts.system_program.clone(),
        ]
    }
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct RegisterCurrencyIxArgs {
    pub royalty: u64,
}

#[derive(Copy, Clone, Debug)]
pub struct RegisterCurrencyIxData<'me>(pub &'me RegisterCurrencyIxArgs);

pub const REGISTER_CURRENCY_IX_DISCM: [u8; 8] = [247, 229, 115, 204, 45, 36, 179, 104];

impl<'me> From<&'me RegisterCurrencyIxArgs> for RegisterCurrencyIxData<'me> {
    fn from(args: &'me RegisterCurrencyIxArgs) -> Self {
        Self(args)
    }
}

impl BorshSerialize for RegisterCurrencyIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&REGISTER_CURRENCY_IX_DISCM)?;
        self.0.serialize(writer)
    }
}

pub fn register_currency_ix<
    K: Into<RegisterCurrencyKeys>,
    A: Into<RegisterCurrencyIxArgs>,
>(accounts: K, args: A) -> std::io::Result<Instruction> {
    let keys: RegisterCurrencyKeys = accounts.into();
    let metas: [AccountMeta; REGISTER_CURRENCY_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: RegisterCurrencyIxArgs = args.into();
    let data: RegisterCurrencyIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}

pub fn register_currency_invoke<'a, A: Into<RegisterCurrencyIxArgs>>(
    accounts: &RegisterCurrencyAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a>,
    args: A,
) -> ProgramResult {
    let ix = register_currency_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; REGISTER_CURRENCY_IX_ACCOUNTS_LEN] = accounts
        .into();
    invoke(&ix, &account_info)
}

pub fn register_currency_invoke_signed<'a, A: Into<RegisterCurrencyIxArgs>>(
    accounts: &RegisterCurrencyAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = register_currency_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; REGISTER_CURRENCY_IX_ACCOUNTS_LEN] = accounts
        .into();
    invoke_signed(&ix, &account_info, seeds)
}

pub const UPDATE_CURRENCY_VAULT_IX_ACCOUNTS_LEN: usize = 6usize;

#[derive(Copy, Clone, Debug)]
pub struct UpdateCurrencyVaultAccounts<
    'me,
    'a0: 'me,
    'a1: 'me,
    'a2: 'me,
    'a3: 'me,
    'a4: 'me,
    'a5: 'me,
> {
    pub update_authority_account: &'me AccountInfo<'a0>,
    pub market_vars_account: &'me AccountInfo<'a1>,
    pub registered_currency: &'me AccountInfo<'a2>,
    pub currency_mint: &'me AccountInfo<'a3>,
    pub sa_currency_vault: &'me AccountInfo<'a4>,
    pub system_program: &'me AccountInfo<'a5>,
}

#[derive(Copy, Clone, Debug)]
pub struct UpdateCurrencyVaultKeys {
    pub update_authority_account: Pubkey,
    pub market_vars_account: Pubkey,
    pub registered_currency: Pubkey,
    pub currency_mint: Pubkey,
    pub sa_currency_vault: Pubkey,
    pub system_program: Pubkey,
}

impl<'me> From<&UpdateCurrencyVaultAccounts<'me, '_, '_, '_, '_, '_, '_>>
for UpdateCurrencyVaultKeys {
    fn from(
        accounts: &UpdateCurrencyVaultAccounts<'me, '_, '_, '_, '_, '_, '_>,
    ) -> Self {
        Self {
            update_authority_account: *accounts.update_authority_account.key,
            market_vars_account: *accounts.market_vars_account.key,
            registered_currency: *accounts.registered_currency.key,
            currency_mint: *accounts.currency_mint.key,
            sa_currency_vault: *accounts.sa_currency_vault.key,
            system_program: *accounts.system_program.key,
        }
    }
}

impl From<&UpdateCurrencyVaultKeys>
for [AccountMeta; UPDATE_CURRENCY_VAULT_IX_ACCOUNTS_LEN] {
    fn from(keys: &UpdateCurrencyVaultKeys) -> Self {
        [
            AccountMeta::new(keys.update_authority_account, true),
            AccountMeta::new_readonly(keys.market_vars_account, false),
            AccountMeta::new(keys.registered_currency, false),
            AccountMeta::new_readonly(keys.currency_mint, false),
            AccountMeta::new_readonly(keys.sa_currency_vault, false),
            AccountMeta::new_readonly(keys.system_program, false),
        ]
    }
}

impl<'a> From<&UpdateCurrencyVaultAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a>>
for [AccountInfo<'a>; UPDATE_CURRENCY_VAULT_IX_ACCOUNTS_LEN] {
    fn from(accounts: &UpdateCurrencyVaultAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a>) -> Self {
        [
            accounts.update_authority_account.clone(),
            accounts.market_vars_account.clone(),
            accounts.registered_currency.clone(),
            accounts.currency_mint.clone(),
            accounts.sa_currency_vault.clone(),
            accounts.system_program.clone(),
        ]
    }
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct UpdateCurrencyVaultIxArgs {}

#[derive(Copy, Clone, Debug)]
pub struct UpdateCurrencyVaultIxData<'me>(pub &'me UpdateCurrencyVaultIxArgs);

pub const UPDATE_CURRENCY_VAULT_IX_DISCM: [u8; 8] = [18, 136, 72, 31, 76, 242, 10, 82];

impl<'me> From<&'me UpdateCurrencyVaultIxArgs> for UpdateCurrencyVaultIxData<'me> {
    fn from(args: &'me UpdateCurrencyVaultIxArgs) -> Self {
        Self(args)
    }
}

impl BorshSerialize for UpdateCurrencyVaultIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_CURRENCY_VAULT_IX_DISCM)?;
        self.0.serialize(writer)
    }
}

pub fn update_currency_vault_ix<
    K: Into<UpdateCurrencyVaultKeys>,
    A: Into<UpdateCurrencyVaultIxArgs>,
>(accounts: K, args: A) -> std::io::Result<Instruction> {
    let keys: UpdateCurrencyVaultKeys = accounts.into();
    let metas: [AccountMeta; UPDATE_CURRENCY_VAULT_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: UpdateCurrencyVaultIxArgs = args.into();
    let data: UpdateCurrencyVaultIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}

pub fn update_currency_vault_invoke<'a, A: Into<UpdateCurrencyVaultIxArgs>>(
    accounts: &UpdateCurrencyVaultAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a>,
    args: A,
) -> ProgramResult {
    let ix = update_currency_vault_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; UPDATE_CURRENCY_VAULT_IX_ACCOUNTS_LEN] = accounts
        .into();
    invoke(&ix, &account_info)
}

pub fn update_currency_vault_invoke_signed<'a, A: Into<UpdateCurrencyVaultIxArgs>>(
    accounts: &UpdateCurrencyVaultAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = update_currency_vault_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; UPDATE_CURRENCY_VAULT_IX_ACCOUNTS_LEN] = accounts
        .into();
    invoke_signed(&ix, &account_info, seeds)
}

pub const UPDATE_CURRENCY_ROYALTY_IX_ACCOUNTS_LEN: usize = 5usize;

#[derive(Copy, Clone, Debug)]
pub struct UpdateCurrencyRoyaltyAccounts<
    'me,
    'a0: 'me,
    'a1: 'me,
    'a2: 'me,
    'a3: 'me,
    'a4: 'me,
> {
    pub update_authority_account: &'me AccountInfo<'a0>,
    pub market_vars_account: &'me AccountInfo<'a1>,
    pub registered_currency: &'me AccountInfo<'a2>,
    pub currency_mint: &'me AccountInfo<'a3>,
    pub system_program: &'me AccountInfo<'a4>,
}

#[derive(Copy, Clone, Debug)]
pub struct UpdateCurrencyRoyaltyKeys {
    pub update_authority_account: Pubkey,
    pub market_vars_account: Pubkey,
    pub registered_currency: Pubkey,
    pub currency_mint: Pubkey,
    pub system_program: Pubkey,
}

impl<'me> From<&UpdateCurrencyRoyaltyAccounts<'me, '_, '_, '_, '_, '_>>
for UpdateCurrencyRoyaltyKeys {
    fn from(accounts: &UpdateCurrencyRoyaltyAccounts<'me, '_, '_, '_, '_, '_>) -> Self {
        Self {
            update_authority_account: *accounts.update_authority_account.key,
            market_vars_account: *accounts.market_vars_account.key,
            registered_currency: *accounts.registered_currency.key,
            currency_mint: *accounts.currency_mint.key,
            system_program: *accounts.system_program.key,
        }
    }
}

impl From<&UpdateCurrencyRoyaltyKeys>
for [AccountMeta; UPDATE_CURRENCY_ROYALTY_IX_ACCOUNTS_LEN] {
    fn from(keys: &UpdateCurrencyRoyaltyKeys) -> Self {
        [
            AccountMeta::new(keys.update_authority_account, true),
            AccountMeta::new_readonly(keys.market_vars_account, false),
            AccountMeta::new(keys.registered_currency, false),
            AccountMeta::new_readonly(keys.currency_mint, false),
            AccountMeta::new_readonly(keys.system_program, false),
        ]
    }
}

impl<'a> From<&UpdateCurrencyRoyaltyAccounts<'_, 'a, 'a, 'a, 'a, 'a>>
for [AccountInfo<'a>; UPDATE_CURRENCY_ROYALTY_IX_ACCOUNTS_LEN] {
    fn from(accounts: &UpdateCurrencyRoyaltyAccounts<'_, 'a, 'a, 'a, 'a, 'a>) -> Self {
        [
            accounts.update_authority_account.clone(),
            accounts.market_vars_account.clone(),
            accounts.registered_currency.clone(),
            accounts.currency_mint.clone(),
            accounts.system_program.clone(),
        ]
    }
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct UpdateCurrencyRoyaltyIxArgs {
    pub royalty: u64,
}

#[derive(Copy, Clone, Debug)]
pub struct UpdateCurrencyRoyaltyIxData<'me>(pub &'me UpdateCurrencyRoyaltyIxArgs);

pub const UPDATE_CURRENCY_ROYALTY_IX_DISCM: [u8; 8] = [
    179,
    232,
    5,
    42,
    204,
    90,
    174,
    248,
];

impl<'me> From<&'me UpdateCurrencyRoyaltyIxArgs> for UpdateCurrencyRoyaltyIxData<'me> {
    fn from(args: &'me UpdateCurrencyRoyaltyIxArgs) -> Self {
        Self(args)
    }
}

impl BorshSerialize for UpdateCurrencyRoyaltyIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_CURRENCY_ROYALTY_IX_DISCM)?;
        self.0.serialize(writer)
    }
}

pub fn update_currency_royalty_ix<
    K: Into<UpdateCurrencyRoyaltyKeys>,
    A: Into<UpdateCurrencyRoyaltyIxArgs>,
>(accounts: K, args: A) -> std::io::Result<Instruction> {
    let keys: UpdateCurrencyRoyaltyKeys = accounts.into();
    let metas: [AccountMeta; UPDATE_CURRENCY_ROYALTY_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: UpdateCurrencyRoyaltyIxArgs = args.into();
    let data: UpdateCurrencyRoyaltyIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}

pub fn update_currency_royalty_invoke<'a, A: Into<UpdateCurrencyRoyaltyIxArgs>>(
    accounts: &UpdateCurrencyRoyaltyAccounts<'_, 'a, 'a, 'a, 'a, 'a>,
    args: A,
) -> ProgramResult {
    let ix = update_currency_royalty_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; UPDATE_CURRENCY_ROYALTY_IX_ACCOUNTS_LEN] = accounts
        .into();
    invoke(&ix, &account_info)
}

pub fn update_currency_royalty_invoke_signed<'a, A: Into<UpdateCurrencyRoyaltyIxArgs>>(
    accounts: &UpdateCurrencyRoyaltyAccounts<'_, 'a, 'a, 'a, 'a, 'a>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = update_currency_royalty_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; UPDATE_CURRENCY_ROYALTY_IX_ACCOUNTS_LEN] = accounts
        .into();
    invoke_signed(&ix, &account_info, seeds)
}

pub const UPDATE_ROYALTY_TIER_IX_ACCOUNTS_LEN: usize = 3usize;

#[derive(Copy, Clone, Debug)]
pub struct UpdateRoyaltyTierAccounts<'me, 'a0: 'me, 'a1: 'me, 'a2: 'me> {
    pub update_authority_account: &'me AccountInfo<'a0>,
    pub market_vars_account: &'me AccountInfo<'a1>,
    pub registered_currency: &'me AccountInfo<'a2>,
}

#[derive(Copy, Clone, Debug)]
pub struct UpdateRoyaltyTierKeys {
    pub update_authority_account: Pubkey,
    pub market_vars_account: Pubkey,
    pub registered_currency: Pubkey,
}

impl<'me> From<&UpdateRoyaltyTierAccounts<'me, '_, '_, '_>> for UpdateRoyaltyTierKeys {
    fn from(accounts: &UpdateRoyaltyTierAccounts<'me, '_, '_, '_>) -> Self {
        Self {
            update_authority_account: *accounts.update_authority_account.key,
            market_vars_account: *accounts.market_vars_account.key,
            registered_currency: *accounts.registered_currency.key,
        }
    }
}

impl From<&UpdateRoyaltyTierKeys>
for [AccountMeta; UPDATE_ROYALTY_TIER_IX_ACCOUNTS_LEN] {
    fn from(keys: &UpdateRoyaltyTierKeys) -> Self {
        [
            AccountMeta::new(keys.update_authority_account, true),
            AccountMeta::new_readonly(keys.market_vars_account, false),
            AccountMeta::new(keys.registered_currency, false),
        ]
    }
}

impl<'a> From<&UpdateRoyaltyTierAccounts<'_, 'a, 'a, 'a>>
for [AccountInfo<'a>; UPDATE_ROYALTY_TIER_IX_ACCOUNTS_LEN] {
    fn from(accounts: &UpdateRoyaltyTierAccounts<'_, 'a, 'a, 'a>) -> Self {
        [
            accounts.update_authority_account.clone(),
            accounts.market_vars_account.clone(),
            accounts.registered_currency.clone(),
        ]
    }
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct UpdateRoyaltyTierIxArgs {
    pub stake_amount: u64,
    pub discount: u64,
}

#[derive(Copy, Clone, Debug)]
pub struct UpdateRoyaltyTierIxData<'me>(pub &'me UpdateRoyaltyTierIxArgs);

pub const UPDATE_ROYALTY_TIER_IX_DISCM: [u8; 8] = [
    123,
    112,
    59,
    126,
    204,
    180,
    191,
    178,
];

impl<'me> From<&'me UpdateRoyaltyTierIxArgs> for UpdateRoyaltyTierIxData<'me> {
    fn from(args: &'me UpdateRoyaltyTierIxArgs) -> Self {
        Self(args)
    }
}

impl BorshSerialize for UpdateRoyaltyTierIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_ROYALTY_TIER_IX_DISCM)?;
        self.0.serialize(writer)
    }
}

pub fn update_royalty_tier_ix<
    K: Into<UpdateRoyaltyTierKeys>,
    A: Into<UpdateRoyaltyTierIxArgs>,
>(accounts: K, args: A) -> std::io::Result<Instruction> {
    let keys: UpdateRoyaltyTierKeys = accounts.into();
    let metas: [AccountMeta; UPDATE_ROYALTY_TIER_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: UpdateRoyaltyTierIxArgs = args.into();
    let data: UpdateRoyaltyTierIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}

pub fn update_royalty_tier_invoke<'a, A: Into<UpdateRoyaltyTierIxArgs>>(
    accounts: &UpdateRoyaltyTierAccounts<'_, 'a, 'a, 'a>,
    args: A,
) -> ProgramResult {
    let ix = update_royalty_tier_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; UPDATE_ROYALTY_TIER_IX_ACCOUNTS_LEN] = accounts
        .into();
    invoke(&ix, &account_info)
}

pub fn update_royalty_tier_invoke_signed<'a, A: Into<UpdateRoyaltyTierIxArgs>>(
    accounts: &UpdateRoyaltyTierAccounts<'_, 'a, 'a, 'a>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = update_royalty_tier_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; UPDATE_ROYALTY_TIER_IX_ACCOUNTS_LEN] = accounts
        .into();
    invoke_signed(&ix, &account_info, seeds)
}

pub const PROCESS_INITIALIZE_BUY_IX_ACCOUNTS_LEN: usize = 14usize;

#[derive(Copy, Clone, Debug)]
pub struct ProcessInitializeBuyAccounts<
    'me,
    'a0: 'me,
    'a1: 'me,
    'a2: 'me,
    'a3: 'me,
    'a4: 'me,
    'a5: 'me,
    'a6: 'me,
    'a7: 'me,
    'a8: 'me,
    'a9: 'me,
    'a10: 'me,
    'a11: 'me,
    'a12: 'me,
    'a13: 'me,
> {
    pub order_initializer: &'me AccountInfo<'a0>,
    pub market_vars_account: &'me AccountInfo<'a1>,
    pub deposit_mint: &'me AccountInfo<'a2>,
    pub receive_mint: &'me AccountInfo<'a3>,
    pub order_vault_account: &'me AccountInfo<'a4>,
    pub order_vault_authority: &'me AccountInfo<'a5>,
    pub initializer_deposit_token_account: &'me AccountInfo<'a6>,
    pub initializer_receive_token_account: &'me AccountInfo<'a7>,
    pub order_account: &'me AccountInfo<'a8>,
    pub registered_currency: &'me AccountInfo<'a9>,
    pub open_orders_counter: &'me AccountInfo<'a10>,
    pub system_program: &'me AccountInfo<'a11>,
    pub rent: &'me AccountInfo<'a12>,
    pub token_program: &'me AccountInfo<'a13>,
}

#[derive(Copy, Clone, Debug)]
pub struct ProcessInitializeBuyKeys {
    pub order_initializer: Pubkey,
    pub market_vars_account: Pubkey,
    pub deposit_mint: Pubkey,
    pub receive_mint: Pubkey,
    pub order_vault_account: Pubkey,
    pub order_vault_authority: Pubkey,
    pub initializer_deposit_token_account: Pubkey,
    pub initializer_receive_token_account: Pubkey,
    pub order_account: Pubkey,
    pub registered_currency: Pubkey,
    pub open_orders_counter: Pubkey,
    pub system_program: Pubkey,
    pub rent: Pubkey,
    pub token_program: Pubkey,
}

impl<
    'me,
> From<
    &ProcessInitializeBuyAccounts<
        'me,
        '_,
        '_,
        '_,
        '_,
        '_,
        '_,
        '_,
        '_,
        '_,
        '_,
        '_,
        '_,
        '_,
        '_,
    >,
> for ProcessInitializeBuyKeys {
    fn from(
        accounts: &ProcessInitializeBuyAccounts<
            'me,
            '_,
            '_,
            '_,
            '_,
            '_,
            '_,
            '_,
            '_,
            '_,
            '_,
            '_,
            '_,
            '_,
            '_,
        >,
    ) -> Self {
        Self {
            order_initializer: *accounts.order_initializer.key,
            market_vars_account: *accounts.market_vars_account.key,
            deposit_mint: *accounts.deposit_mint.key,
            receive_mint: *accounts.receive_mint.key,
            order_vault_account: *accounts.order_vault_account.key,
            order_vault_authority: *accounts.order_vault_authority.key,
            initializer_deposit_token_account: *accounts
                .initializer_deposit_token_account
                .key,
            initializer_receive_token_account: *accounts
                .initializer_receive_token_account
                .key,
            order_account: *accounts.order_account.key,
            registered_currency: *accounts.registered_currency.key,
            open_orders_counter: *accounts.open_orders_counter.key,
            system_program: *accounts.system_program.key,
            rent: *accounts.rent.key,
            token_program: *accounts.token_program.key,
        }
    }
}

impl From<&ProcessInitializeBuyKeys>
for [AccountMeta; PROCESS_INITIALIZE_BUY_IX_ACCOUNTS_LEN] {
    fn from(keys: &ProcessInitializeBuyKeys) -> Self {
        [
            AccountMeta::new(keys.order_initializer, true),
            AccountMeta::new_readonly(keys.market_vars_account, false),
            AccountMeta::new_readonly(keys.deposit_mint, false),
            AccountMeta::new_readonly(keys.receive_mint, false),
            AccountMeta::new(keys.order_vault_account, false),
            AccountMeta::new_readonly(keys.order_vault_authority, false),
            AccountMeta::new(keys.initializer_deposit_token_account, false),
            AccountMeta::new(keys.initializer_receive_token_account, false),
            AccountMeta::new(keys.order_account, false),
            AccountMeta::new_readonly(keys.registered_currency, false),
            AccountMeta::new(keys.open_orders_counter, false),
            AccountMeta::new_readonly(keys.system_program, false),
            AccountMeta::new_readonly(keys.rent, false),
            AccountMeta::new_readonly(keys.token_program, false),
        ]
    }
}

impl<
    'a,
> From<
    &ProcessInitializeBuyAccounts<
        '_,
        'a,
        'a,
        'a,
        'a,
        'a,
        'a,
        'a,
        'a,
        'a,
        'a,
        'a,
        'a,
        'a,
        'a,
    >,
> for [AccountInfo<'a>; PROCESS_INITIALIZE_BUY_IX_ACCOUNTS_LEN] {
    fn from(
        accounts: &ProcessInitializeBuyAccounts<
            '_,
            'a,
            'a,
            'a,
            'a,
            'a,
            'a,
            'a,
            'a,
            'a,
            'a,
            'a,
            'a,
            'a,
            'a,
        >,
    ) -> Self {
        [
            accounts.order_initializer.clone(),
            accounts.market_vars_account.clone(),
            accounts.deposit_mint.clone(),
            accounts.receive_mint.clone(),
            accounts.order_vault_account.clone(),
            accounts.order_vault_authority.clone(),
            accounts.initializer_deposit_token_account.clone(),
            accounts.initializer_receive_token_account.clone(),
            accounts.order_account.clone(),
            accounts.registered_currency.clone(),
            accounts.open_orders_counter.clone(),
            accounts.system_program.clone(),
            accounts.rent.clone(),
            accounts.token_program.clone(),
        ]
    }
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct ProcessInitializeBuyIxArgs {
    pub price: u64,
    pub origination_qty: u64,
}

#[derive(Copy, Clone, Debug)]
pub struct ProcessInitializeBuyIxData<'me>(pub &'me ProcessInitializeBuyIxArgs);

pub const PROCESS_INITIALIZE_BUY_IX_DISCM: [u8; 8] = [
    129,
    142,
    102,
    190,
    138,
    103,
    145,
    131,
];

impl<'me> From<&'me ProcessInitializeBuyIxArgs> for ProcessInitializeBuyIxData<'me> {
    fn from(args: &'me ProcessInitializeBuyIxArgs) -> Self {
        Self(args)
    }
}

impl BorshSerialize for ProcessInitializeBuyIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&PROCESS_INITIALIZE_BUY_IX_DISCM)?;
        self.0.serialize(writer)
    }
}

pub fn process_initialize_buy_ix<
    K: Into<ProcessInitializeBuyKeys>,
    A: Into<ProcessInitializeBuyIxArgs>,
>(accounts: K, args: A) -> std::io::Result<Instruction> {
    let keys: ProcessInitializeBuyKeys = accounts.into();
    let metas: [AccountMeta; PROCESS_INITIALIZE_BUY_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: ProcessInitializeBuyIxArgs = args.into();
    let data: ProcessInitializeBuyIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}

pub fn process_initialize_buy_invoke<'a, A: Into<ProcessInitializeBuyIxArgs>>(
    accounts: &ProcessInitializeBuyAccounts<
        '_,
        'a,
        'a,
        'a,
        'a,
        'a,
        'a,
        'a,
        'a,
        'a,
        'a,
        'a,
        'a,
        'a,
        'a,
    >,
    args: A,
) -> ProgramResult {
    let ix = process_initialize_buy_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; PROCESS_INITIALIZE_BUY_IX_ACCOUNTS_LEN] = accounts
        .into();
    invoke(&ix, &account_info)
}

pub fn process_initialize_buy_invoke_signed<'a, A: Into<ProcessInitializeBuyIxArgs>>(
    accounts: &ProcessInitializeBuyAccounts<
        '_,
        'a,
        'a,
        'a,
        'a,
        'a,
        'a,
        'a,
        'a,
        'a,
        'a,
        'a,
        'a,
        'a,
        'a,
    >,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = process_initialize_buy_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; PROCESS_INITIALIZE_BUY_IX_ACCOUNTS_LEN] = accounts
        .into();
    invoke_signed(&ix, &account_info, seeds)
}

pub const PROCESS_INITIALIZE_SELL_IX_ACCOUNTS_LEN: usize = 14usize;

#[derive(Copy, Clone, Debug)]
pub struct ProcessInitializeSellAccounts<
    'me,
    'a0: 'me,
    'a1: 'me,
    'a2: 'me,
    'a3: 'me,
    'a4: 'me,
    'a5: 'me,
    'a6: 'me,
    'a7: 'me,
    'a8: 'me,
    'a9: 'me,
    'a10: 'me,
    'a11: 'me,
    'a12: 'me,
    'a13: 'me,
> {
    pub order_initializer: &'me AccountInfo<'a0>,
    pub market_vars_account: &'me AccountInfo<'a1>,
    pub deposit_mint: &'me AccountInfo<'a2>,
    pub receive_mint: &'me AccountInfo<'a3>,
    pub order_vault_account: &'me AccountInfo<'a4>,
    pub order_vault_authority: &'me AccountInfo<'a5>,
    pub initializer_deposit_token_account: &'me AccountInfo<'a6>,
    pub initializer_receive_token_account: &'me AccountInfo<'a7>,
    pub order_account: &'me AccountInfo<'a8>,
    pub registered_currency: &'me AccountInfo<'a9>,
    pub open_orders_counter: &'me AccountInfo<'a10>,
    pub system_program: &'me AccountInfo<'a11>,
    pub rent: &'me AccountInfo<'a12>,
    pub token_program: &'me AccountInfo<'a13>,
}

#[derive(Copy, Clone, Debug)]
pub struct ProcessInitializeSellKeys {
    pub order_initializer: Pubkey,
    pub market_vars_account: Pubkey,
    pub deposit_mint: Pubkey,
    pub receive_mint: Pubkey,
    pub order_vault_account: Pubkey,
    pub order_vault_authority: Pubkey,
    pub initializer_deposit_token_account: Pubkey,
    pub initializer_receive_token_account: Pubkey,
    pub order_account: Pubkey,
    pub registered_currency: Pubkey,
    pub open_orders_counter: Pubkey,
    pub system_program: Pubkey,
    pub rent: Pubkey,
    pub token_program: Pubkey,
}

impl<
    'me,
> From<
    &ProcessInitializeSellAccounts<
        'me,
        '_,
        '_,
        '_,
        '_,
        '_,
        '_,
        '_,
        '_,
        '_,
        '_,
        '_,
        '_,
        '_,
        '_,
    >,
> for ProcessInitializeSellKeys {
    fn from(
        accounts: &ProcessInitializeSellAccounts<
            'me,
            '_,
            '_,
            '_,
            '_,
            '_,
            '_,
            '_,
            '_,
            '_,
            '_,
            '_,
            '_,
            '_,
            '_,
        >,
    ) -> Self {
        Self {
            order_initializer: *accounts.order_initializer.key,
            market_vars_account: *accounts.market_vars_account.key,
            deposit_mint: *accounts.deposit_mint.key,
            receive_mint: *accounts.receive_mint.key,
            order_vault_account: *accounts.order_vault_account.key,
            order_vault_authority: *accounts.order_vault_authority.key,
            initializer_deposit_token_account: *accounts
                .initializer_deposit_token_account
                .key,
            initializer_receive_token_account: *accounts
                .initializer_receive_token_account
                .key,
            order_account: *accounts.order_account.key,
            registered_currency: *accounts.registered_currency.key,
            open_orders_counter: *accounts.open_orders_counter.key,
            system_program: *accounts.system_program.key,
            rent: *accounts.rent.key,
            token_program: *accounts.token_program.key,
        }
    }
}

impl From<&ProcessInitializeSellKeys>
for [AccountMeta; PROCESS_INITIALIZE_SELL_IX_ACCOUNTS_LEN] {
    fn from(keys: &ProcessInitializeSellKeys) -> Self {
        [
            AccountMeta::new(keys.order_initializer, true),
            AccountMeta::new_readonly(keys.market_vars_account, false),
            AccountMeta::new_readonly(keys.deposit_mint, false),
            AccountMeta::new_readonly(keys.receive_mint, false),
            AccountMeta::new(keys.order_vault_account, false),
            AccountMeta::new_readonly(keys.order_vault_authority, false),
            AccountMeta::new(keys.initializer_deposit_token_account, false),
            AccountMeta::new_readonly(keys.initializer_receive_token_account, false),
            AccountMeta::new(keys.order_account, false),
            AccountMeta::new_readonly(keys.registered_currency, false),
            AccountMeta::new(keys.open_orders_counter, false),
            AccountMeta::new_readonly(keys.system_program, false),
            AccountMeta::new_readonly(keys.rent, false),
            AccountMeta::new_readonly(keys.token_program, false),
        ]
    }
}

impl<
    'a,
> From<
    &ProcessInitializeSellAccounts<
        '_,
        'a,
        'a,
        'a,
        'a,
        'a,
        'a,
        'a,
        'a,
        'a,
        'a,
        'a,
        'a,
        'a,
        'a,
    >,
> for [AccountInfo<'a>; PROCESS_INITIALIZE_SELL_IX_ACCOUNTS_LEN] {
    fn from(
        accounts: &ProcessInitializeSellAccounts<
            '_,
            'a,
            'a,
            'a,
            'a,
            'a,
            'a,
            'a,
            'a,
            'a,
            'a,
            'a,
            'a,
            'a,
            'a,
        >,
    ) -> Self {
        [
            accounts.order_initializer.clone(),
            accounts.market_vars_account.clone(),
            accounts.deposit_mint.clone(),
            accounts.receive_mint.clone(),
            accounts.order_vault_account.clone(),
            accounts.order_vault_authority.clone(),
            accounts.initializer_deposit_token_account.clone(),
            accounts.initializer_receive_token_account.clone(),
            accounts.order_account.clone(),
            accounts.registered_currency.clone(),
            accounts.open_orders_counter.clone(),
            accounts.system_program.clone(),
            accounts.rent.clone(),
            accounts.token_program.clone(),
        ]
    }
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct ProcessInitializeSellIxArgs {
    pub price: u64,
    pub origination_qty: u64,
}

#[derive(Copy, Clone, Debug)]
pub struct ProcessInitializeSellIxData<'me>(pub &'me ProcessInitializeSellIxArgs);

pub const PROCESS_INITIALIZE_SELL_IX_DISCM: [u8; 8] = [
    43,
    42,
    167,
    252,
    25,
    47,
    212,
    225,
];

impl<'me> From<&'me ProcessInitializeSellIxArgs> for ProcessInitializeSellIxData<'me> {
    fn from(args: &'me ProcessInitializeSellIxArgs) -> Self {
        Self(args)
    }
}

impl BorshSerialize for ProcessInitializeSellIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&PROCESS_INITIALIZE_SELL_IX_DISCM)?;
        self.0.serialize(writer)
    }
}

pub fn process_initialize_sell_ix<
    K: Into<ProcessInitializeSellKeys>,
    A: Into<ProcessInitializeSellIxArgs>,
>(accounts: K, args: A) -> std::io::Result<Instruction> {
    let keys: ProcessInitializeSellKeys = accounts.into();
    let metas: [AccountMeta; PROCESS_INITIALIZE_SELL_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: ProcessInitializeSellIxArgs = args.into();
    let data: ProcessInitializeSellIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}

pub fn process_initialize_sell_invoke<'a, A: Into<ProcessInitializeSellIxArgs>>(
    accounts: &ProcessInitializeSellAccounts<
        '_,
        'a,
        'a,
        'a,
        'a,
        'a,
        'a,
        'a,
        'a,
        'a,
        'a,
        'a,
        'a,
        'a,
        'a,
    >,
    args: A,
) -> ProgramResult {
    let ix = process_initialize_sell_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; PROCESS_INITIALIZE_SELL_IX_ACCOUNTS_LEN] = accounts
        .into();
    invoke(&ix, &account_info)
}

pub fn process_initialize_sell_invoke_signed<'a, A: Into<ProcessInitializeSellIxArgs>>(
    accounts: &ProcessInitializeSellAccounts<
        '_,
        'a,
        'a,
        'a,
        'a,
        'a,
        'a,
        'a,
        'a,
        'a,
        'a,
        'a,
        'a,
        'a,
        'a,
    >,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = process_initialize_sell_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; PROCESS_INITIALIZE_SELL_IX_ACCOUNTS_LEN] = accounts
        .into();
    invoke_signed(&ix, &account_info, seeds)
}

pub const PROCESS_EXCHANGE_IX_ACCOUNTS_LEN: usize = 18usize;

#[derive(Copy, Clone, Debug)]
pub struct ProcessExchangeAccounts<
    'me,
    'a0: 'me,
    'a1: 'me,
    'a2: 'me,
    'a3: 'me,
    'a4: 'me,
    'a5: 'me,
    'a6: 'me,
    'a7: 'me,
    'a8: 'me,
    'a9: 'me,
    'a10: 'me,
    'a11: 'me,
    'a12: 'me,
    'a13: 'me,
    'a14: 'me,
    'a15: 'me,
    'a16: 'me,
    'a17: 'me,
> {
    pub order_taker: &'me AccountInfo<'a0>,
    pub order_taker_deposit_token_account: &'me AccountInfo<'a1>,
    pub order_taker_receive_token_account: &'me AccountInfo<'a2>,
    pub currency_mint: &'me AccountInfo<'a3>,
    pub asset_mint: &'me AccountInfo<'a4>,
    pub order_initializer: &'me AccountInfo<'a5>,
    pub initializer_deposit_token_account: &'me AccountInfo<'a6>,
    pub initializer_receive_token_account: &'me AccountInfo<'a7>,
    pub order_vault_account: &'me AccountInfo<'a8>,
    pub order_vault_authority: &'me AccountInfo<'a9>,
    pub order_account: &'me AccountInfo<'a10>,
    pub sa_vault: &'me AccountInfo<'a11>,
    pub registered_currency: &'me AccountInfo<'a12>,
    pub open_orders_counter: &'me AccountInfo<'a13>,
    pub token_program: &'me AccountInfo<'a14>,
    pub atlas_staking: &'me AccountInfo<'a15>,
    pub registered_stake: &'me AccountInfo<'a16>,
    pub staking_account: &'me AccountInfo<'a17>,
}

#[derive(Copy, Clone, Debug)]
pub struct ProcessExchangeKeys {
    pub order_taker: Pubkey,
    pub order_taker_deposit_token_account: Pubkey,
    pub order_taker_receive_token_account: Pubkey,
    pub currency_mint: Pubkey,
    pub asset_mint: Pubkey,
    pub order_initializer: Pubkey,
    pub initializer_deposit_token_account: Pubkey,
    pub initializer_receive_token_account: Pubkey,
    pub order_vault_account: Pubkey,
    pub order_vault_authority: Pubkey,
    pub order_account: Pubkey,
    pub sa_vault: Pubkey,
    pub registered_currency: Pubkey,
    pub open_orders_counter: Pubkey,
    pub token_program: Pubkey,
    pub atlas_staking: Pubkey,
    pub registered_stake: Pubkey,
    pub staking_account: Pubkey,
}

impl<
    'me,
> From<
    &ProcessExchangeAccounts<
        'me,
        '_,
        '_,
        '_,
        '_,
        '_,
        '_,
        '_,
        '_,
        '_,
        '_,
        '_,
        '_,
        '_,
        '_,
        '_,
        '_,
        '_,
        '_,
    >,
> for ProcessExchangeKeys {
    fn from(
        accounts: &ProcessExchangeAccounts<
            'me,
            '_,
            '_,
            '_,
            '_,
            '_,
            '_,
            '_,
            '_,
            '_,
            '_,
            '_,
            '_,
            '_,
            '_,
            '_,
            '_,
            '_,
            '_,
        >,
    ) -> Self {
        Self {
            order_taker: *accounts.order_taker.key,
            order_taker_deposit_token_account: *accounts
                .order_taker_deposit_token_account
                .key,
            order_taker_receive_token_account: *accounts
                .order_taker_receive_token_account
                .key,
            currency_mint: *accounts.currency_mint.key,
            asset_mint: *accounts.asset_mint.key,
            order_initializer: *accounts.order_initializer.key,
            initializer_deposit_token_account: *accounts
                .initializer_deposit_token_account
                .key,
            initializer_receive_token_account: *accounts
                .initializer_receive_token_account
                .key,
            order_vault_account: *accounts.order_vault_account.key,
            order_vault_authority: *accounts.order_vault_authority.key,
            order_account: *accounts.order_account.key,
            sa_vault: *accounts.sa_vault.key,
            registered_currency: *accounts.registered_currency.key,
            open_orders_counter: *accounts.open_orders_counter.key,
            token_program: *accounts.token_program.key,
            atlas_staking: *accounts.atlas_staking.key,
            registered_stake: *accounts.registered_stake.key,
            staking_account: *accounts.staking_account.key,
        }
    }
}

impl From<&ProcessExchangeKeys> for [AccountMeta; PROCESS_EXCHANGE_IX_ACCOUNTS_LEN] {
    fn from(keys: &ProcessExchangeKeys) -> Self {
        [
            AccountMeta::new(keys.order_taker, true),
            AccountMeta::new(keys.order_taker_deposit_token_account, false),
            AccountMeta::new(keys.order_taker_receive_token_account, false),
            AccountMeta::new_readonly(keys.currency_mint, false),
            AccountMeta::new_readonly(keys.asset_mint, false),
            AccountMeta::new(keys.order_initializer, false),
            AccountMeta::new(keys.initializer_deposit_token_account, false),
            AccountMeta::new(keys.initializer_receive_token_account, false),
            AccountMeta::new(keys.order_vault_account, false),
            AccountMeta::new_readonly(keys.order_vault_authority, false),
            AccountMeta::new(keys.order_account, false),
            AccountMeta::new(keys.sa_vault, false),
            AccountMeta::new_readonly(keys.registered_currency, false),
            AccountMeta::new(keys.open_orders_counter, false),
            AccountMeta::new_readonly(keys.token_program, false),
            AccountMeta::new_readonly(keys.atlas_staking, false),
            AccountMeta::new_readonly(keys.registered_stake, false),
            AccountMeta::new_readonly(keys.staking_account, false),
        ]
    }
}

impl<
    'a,
> From<
    &ProcessExchangeAccounts<
        '_,
        'a,
        'a,
        'a,
        'a,
        'a,
        'a,
        'a,
        'a,
        'a,
        'a,
        'a,
        'a,
        'a,
        'a,
        'a,
        'a,
        'a,
        'a,
    >,
> for [AccountInfo<'a>; PROCESS_EXCHANGE_IX_ACCOUNTS_LEN] {
    fn from(
        accounts: &ProcessExchangeAccounts<
            '_,
            'a,
            'a,
            'a,
            'a,
            'a,
            'a,
            'a,
            'a,
            'a,
            'a,
            'a,
            'a,
            'a,
            'a,
            'a,
            'a,
            'a,
            'a,
        >,
    ) -> Self {
        [
            accounts.order_taker.clone(),
            accounts.order_taker_deposit_token_account.clone(),
            accounts.order_taker_receive_token_account.clone(),
            accounts.currency_mint.clone(),
            accounts.asset_mint.clone(),
            accounts.order_initializer.clone(),
            accounts.initializer_deposit_token_account.clone(),
            accounts.initializer_receive_token_account.clone(),
            accounts.order_vault_account.clone(),
            accounts.order_vault_authority.clone(),
            accounts.order_account.clone(),
            accounts.sa_vault.clone(),
            accounts.registered_currency.clone(),
            accounts.open_orders_counter.clone(),
            accounts.token_program.clone(),
            accounts.atlas_staking.clone(),
            accounts.registered_stake.clone(),
            accounts.staking_account.clone(),
        ]
    }
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct ProcessExchangeIxArgs {
    pub purchase_quantity: u64,
    pub expected_price: u64,
    pub seller: Pubkey,
}

#[derive(Copy, Clone, Debug)]
pub struct ProcessExchangeIxData<'me>(pub &'me ProcessExchangeIxArgs);

pub const PROCESS_EXCHANGE_IX_DISCM: [u8; 8] = [112, 194, 63, 99, 52, 147, 85, 48];

impl<'me> From<&'me ProcessExchangeIxArgs> for ProcessExchangeIxData<'me> {
    fn from(args: &'me ProcessExchangeIxArgs) -> Self {
        Self(args)
    }
}

impl BorshSerialize for ProcessExchangeIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&PROCESS_EXCHANGE_IX_DISCM)?;
        self.0.serialize(writer)
    }
}

pub fn process_exchange_ix<K: Into<ProcessExchangeKeys>, A: Into<ProcessExchangeIxArgs>>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: ProcessExchangeKeys = accounts.into();
    let metas: [AccountMeta; PROCESS_EXCHANGE_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: ProcessExchangeIxArgs = args.into();
    let data: ProcessExchangeIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}

pub fn process_exchange_invoke<'a, A: Into<ProcessExchangeIxArgs>>(
    accounts: &ProcessExchangeAccounts<
        '_,
        'a,
        'a,
        'a,
        'a,
        'a,
        'a,
        'a,
        'a,
        'a,
        'a,
        'a,
        'a,
        'a,
        'a,
        'a,
        'a,
        'a,
        'a,
    >,
    args: A,
) -> ProgramResult {
    let ix = process_exchange_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; PROCESS_EXCHANGE_IX_ACCOUNTS_LEN] = accounts
        .into();
    invoke(&ix, &account_info)
}

pub fn process_exchange_invoke_signed<'a, A: Into<ProcessExchangeIxArgs>>(
    accounts: &ProcessExchangeAccounts<
        '_,
        'a,
        'a,
        'a,
        'a,
        'a,
        'a,
        'a,
        'a,
        'a,
        'a,
        'a,
        'a,
        'a,
        'a,
        'a,
        'a,
        'a,
        'a,
    >,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = process_exchange_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; PROCESS_EXCHANGE_IX_ACCOUNTS_LEN] = accounts
        .into();
    invoke_signed(&ix, &account_info, seeds)
}

pub const PROCESS_CANCEL_IX_ACCOUNTS_LEN: usize = 10usize;

#[derive(Copy, Clone, Debug)]
pub struct ProcessCancelAccounts<
    'me,
    'a0: 'me,
    'a1: 'me,
    'a2: 'me,
    'a3: 'me,
    'a4: 'me,
    'a5: 'me,
    'a6: 'me,
    'a7: 'me,
    'a8: 'me,
    'a9: 'me,
> {
    pub signer: &'me AccountInfo<'a0>,
    pub order_initializer: &'me AccountInfo<'a1>,
    pub market_vars_account: &'me AccountInfo<'a2>,
    pub deposit_mint: &'me AccountInfo<'a3>,
    pub initializer_deposit_token_account: &'me AccountInfo<'a4>,
    pub order_vault_account: &'me AccountInfo<'a5>,
    pub order_vault_authority: &'me AccountInfo<'a6>,
    pub order_account: &'me AccountInfo<'a7>,
    pub open_orders_counter: &'me AccountInfo<'a8>,
    pub token_program: &'me AccountInfo<'a9>,
}

#[derive(Copy, Clone, Debug)]
pub struct ProcessCancelKeys {
    pub signer: Pubkey,
    pub order_initializer: Pubkey,
    pub market_vars_account: Pubkey,
    pub deposit_mint: Pubkey,
    pub initializer_deposit_token_account: Pubkey,
    pub order_vault_account: Pubkey,
    pub order_vault_authority: Pubkey,
    pub order_account: Pubkey,
    pub open_orders_counter: Pubkey,
    pub token_program: Pubkey,
}

impl<'me> From<&ProcessCancelAccounts<'me, '_, '_, '_, '_, '_, '_, '_, '_, '_, '_>>
for ProcessCancelKeys {
    fn from(
        accounts: &ProcessCancelAccounts<'me, '_, '_, '_, '_, '_, '_, '_, '_, '_, '_>,
    ) -> Self {
        Self {
            signer: *accounts.signer.key,
            order_initializer: *accounts.order_initializer.key,
            market_vars_account: *accounts.market_vars_account.key,
            deposit_mint: *accounts.deposit_mint.key,
            initializer_deposit_token_account: *accounts
                .initializer_deposit_token_account
                .key,
            order_vault_account: *accounts.order_vault_account.key,
            order_vault_authority: *accounts.order_vault_authority.key,
            order_account: *accounts.order_account.key,
            open_orders_counter: *accounts.open_orders_counter.key,
            token_program: *accounts.token_program.key,
        }
    }
}

impl From<&ProcessCancelKeys> for [AccountMeta; PROCESS_CANCEL_IX_ACCOUNTS_LEN] {
    fn from(keys: &ProcessCancelKeys) -> Self {
        [
            AccountMeta::new(keys.signer, true),
            AccountMeta::new(keys.order_initializer, false),
            AccountMeta::new_readonly(keys.market_vars_account, false),
            AccountMeta::new_readonly(keys.deposit_mint, false),
            AccountMeta::new(keys.initializer_deposit_token_account, false),
            AccountMeta::new(keys.order_vault_account, false),
            AccountMeta::new_readonly(keys.order_vault_authority, false),
            AccountMeta::new(keys.order_account, false),
            AccountMeta::new(keys.open_orders_counter, false),
            AccountMeta::new_readonly(keys.token_program, false),
        ]
    }
}

impl<'a> From<&ProcessCancelAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a>>
for [AccountInfo<'a>; PROCESS_CANCEL_IX_ACCOUNTS_LEN] {
    fn from(
        accounts: &ProcessCancelAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a>,
    ) -> Self {
        [
            accounts.signer.clone(),
            accounts.order_initializer.clone(),
            accounts.market_vars_account.clone(),
            accounts.deposit_mint.clone(),
            accounts.initializer_deposit_token_account.clone(),
            accounts.order_vault_account.clone(),
            accounts.order_vault_authority.clone(),
            accounts.order_account.clone(),
            accounts.open_orders_counter.clone(),
            accounts.token_program.clone(),
        ]
    }
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct ProcessCancelIxArgs {}

#[derive(Copy, Clone, Debug)]
pub struct ProcessCancelIxData<'me>(pub &'me ProcessCancelIxArgs);

pub const PROCESS_CANCEL_IX_DISCM: [u8; 8] = [85, 84, 214, 240, 140, 41, 230, 149];

impl<'me> From<&'me ProcessCancelIxArgs> for ProcessCancelIxData<'me> {
    fn from(args: &'me ProcessCancelIxArgs) -> Self {
        Self(args)
    }
}

impl BorshSerialize for ProcessCancelIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&PROCESS_CANCEL_IX_DISCM)?;
        self.0.serialize(writer)
    }
}

pub fn process_cancel_ix<K: Into<ProcessCancelKeys>, A: Into<ProcessCancelIxArgs>>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: ProcessCancelKeys = accounts.into();
    let metas: [AccountMeta; PROCESS_CANCEL_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: ProcessCancelIxArgs = args.into();
    let data: ProcessCancelIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}

pub fn process_cancel_invoke<'a, A: Into<ProcessCancelIxArgs>>(
    accounts: &ProcessCancelAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a>,
    args: A,
) -> ProgramResult {
    let ix = process_cancel_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; PROCESS_CANCEL_IX_ACCOUNTS_LEN] = accounts
        .into();
    invoke(&ix, &account_info)
}

pub fn process_cancel_invoke_signed<'a, A: Into<ProcessCancelIxArgs>>(
    accounts: &ProcessCancelAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = process_cancel_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; PROCESS_CANCEL_IX_ACCOUNTS_LEN] = accounts
        .into();
    invoke_signed(&ix, &account_info, seeds)
}

pub const INITIALIZE_OPEN_ORDERS_COUNTER_IX_ACCOUNTS_LEN: usize = 5usize;

#[derive(Copy, Clone, Debug)]
pub struct InitializeOpenOrdersCounterAccounts<
    'me,
    'a0: 'me,
    'a1: 'me,
    'a2: 'me,
    'a3: 'me,
    'a4: 'me,
> {
    pub payer: &'me AccountInfo<'a0>,
    pub user: &'me AccountInfo<'a1>,
    pub open_orders_counter: &'me AccountInfo<'a2>,
    pub deposit_mint: &'me AccountInfo<'a3>,
    pub system_program: &'me AccountInfo<'a4>,
}

#[derive(Copy, Clone, Debug)]
pub struct InitializeOpenOrdersCounterKeys {
    pub payer: Pubkey,
    pub user: Pubkey,
    pub open_orders_counter: Pubkey,
    pub deposit_mint: Pubkey,
    pub system_program: Pubkey,
}

impl<'me> From<&InitializeOpenOrdersCounterAccounts<'me, '_, '_, '_, '_, '_>>
for InitializeOpenOrdersCounterKeys {
    fn from(
        accounts: &InitializeOpenOrdersCounterAccounts<'me, '_, '_, '_, '_, '_>,
    ) -> Self {
        Self {
            payer: *accounts.payer.key,
            user: *accounts.user.key,
            open_orders_counter: *accounts.open_orders_counter.key,
            deposit_mint: *accounts.deposit_mint.key,
            system_program: *accounts.system_program.key,
        }
    }
}

impl From<&InitializeOpenOrdersCounterKeys>
for [AccountMeta; INITIALIZE_OPEN_ORDERS_COUNTER_IX_ACCOUNTS_LEN] {
    fn from(keys: &InitializeOpenOrdersCounterKeys) -> Self {
        [
            AccountMeta::new(keys.payer, true),
            AccountMeta::new_readonly(keys.user, false),
            AccountMeta::new(keys.open_orders_counter, false),
            AccountMeta::new_readonly(keys.deposit_mint, false),
            AccountMeta::new_readonly(keys.system_program, false),
        ]
    }
}

impl<'a> From<&InitializeOpenOrdersCounterAccounts<'_, 'a, 'a, 'a, 'a, 'a>>
for [AccountInfo<'a>; INITIALIZE_OPEN_ORDERS_COUNTER_IX_ACCOUNTS_LEN] {
    fn from(
        accounts: &InitializeOpenOrdersCounterAccounts<'_, 'a, 'a, 'a, 'a, 'a>,
    ) -> Self {
        [
            accounts.payer.clone(),
            accounts.user.clone(),
            accounts.open_orders_counter.clone(),
            accounts.deposit_mint.clone(),
            accounts.system_program.clone(),
        ]
    }
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct InitializeOpenOrdersCounterIxArgs {}

#[derive(Copy, Clone, Debug)]
pub struct InitializeOpenOrdersCounterIxData<'me>(
    pub &'me InitializeOpenOrdersCounterIxArgs,
);

pub const INITIALIZE_OPEN_ORDERS_COUNTER_IX_DISCM: [u8; 8] = [
    221,
    134,
    5,
    76,
    4,
    145,
    202,
    29,
];

impl<'me> From<&'me InitializeOpenOrdersCounterIxArgs>
for InitializeOpenOrdersCounterIxData<'me> {
    fn from(args: &'me InitializeOpenOrdersCounterIxArgs) -> Self {
        Self(args)
    }
}

impl BorshSerialize for InitializeOpenOrdersCounterIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_OPEN_ORDERS_COUNTER_IX_DISCM)?;
        self.0.serialize(writer)
    }
}

pub fn initialize_open_orders_counter_ix<
    K: Into<InitializeOpenOrdersCounterKeys>,
    A: Into<InitializeOpenOrdersCounterIxArgs>,
>(accounts: K, args: A) -> std::io::Result<Instruction> {
    let keys: InitializeOpenOrdersCounterKeys = accounts.into();
    let metas: [AccountMeta; INITIALIZE_OPEN_ORDERS_COUNTER_IX_ACCOUNTS_LEN] = (&keys)
        .into();
    let args_full: InitializeOpenOrdersCounterIxArgs = args.into();
    let data: InitializeOpenOrdersCounterIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}

pub fn initialize_open_orders_counter_invoke<
    'a,
    A: Into<InitializeOpenOrdersCounterIxArgs>,
>(
    accounts: &InitializeOpenOrdersCounterAccounts<'_, 'a, 'a, 'a, 'a, 'a>,
    args: A,
) -> ProgramResult {
    let ix = initialize_open_orders_counter_ix(accounts, args)?;
    let account_info: [AccountInfo<
        'a,
    >; INITIALIZE_OPEN_ORDERS_COUNTER_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}

pub fn initialize_open_orders_counter_invoke_signed<
    'a,
    A: Into<InitializeOpenOrdersCounterIxArgs>,
>(
    accounts: &InitializeOpenOrdersCounterAccounts<'_, 'a, 'a, 'a, 'a, 'a>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = initialize_open_orders_counter_ix(accounts, args)?;
    let account_info: [AccountInfo<
        'a,
    >; INITIALIZE_OPEN_ORDERS_COUNTER_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
