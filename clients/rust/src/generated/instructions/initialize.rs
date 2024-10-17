//! This code was AUTOGENERATED using the codama library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun codama to update it.
//!
//! <https://github.com/codama-idl/codama>

use {
    borsh::{BorshDeserialize, BorshSerialize},
    solana_program::pubkey::Pubkey,
};

/// Accounts.
pub struct Initialize {
    /// The stake account to initialize
    pub stake: solana_program::pubkey::Pubkey,
    /// Rent sysvar
    pub rent: solana_program::pubkey::Pubkey,
}

impl Initialize {
    pub fn instruction(
        &self,
        args: InitializeInstructionArgs,
    ) -> solana_program::instruction::Instruction {
        self.instruction_with_remaining_accounts(args, &[])
    }
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        args: InitializeInstructionArgs,
        remaining_accounts: &[solana_program::instruction::AccountMeta],
    ) -> solana_program::instruction::Instruction {
        let mut accounts = Vec::with_capacity(2 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.stake, false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.rent, false,
        ));
        accounts.extend_from_slice(remaining_accounts);
        let mut data = InitializeInstructionData::new().try_to_vec().unwrap();
        let mut args = args.try_to_vec().unwrap();
        data.append(&mut args);

        solana_program::instruction::Instruction {
            program_id: crate::STAKE_PROGRAM_ID,
            accounts,
            data,
        }
    }
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct InitializeInstructionData {
    discriminator: [u8; 8],
}

impl InitializeInstructionData {
    pub fn new() -> Self {
        Self {
            discriminator: [175, 175, 109, 31, 13, 152, 155, 237],
        }
    }
}

impl Default for InitializeInstructionData {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InitializeInstructionArgs {
    pub staker: Pubkey,
    pub withdrawer: Pubkey,
    pub unix_timestamp: i64,
    pub epoch: u64,
    pub custodian: Pubkey,
}

/// Instruction builder for `Initialize`.
///
/// ### Accounts:
///
///   0. `[writable]` stake
///   1. `[optional]` rent (default to
///      `SysvarRent111111111111111111111111111111111`)
#[derive(Clone, Debug, Default)]
pub struct InitializeBuilder {
    stake: Option<solana_program::pubkey::Pubkey>,
    rent: Option<solana_program::pubkey::Pubkey>,
    staker: Option<Pubkey>,
    withdrawer: Option<Pubkey>,
    unix_timestamp: Option<i64>,
    epoch: Option<u64>,
    custodian: Option<Pubkey>,
    __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl InitializeBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    /// The stake account to initialize
    #[inline(always)]
    pub fn stake(&mut self, stake: solana_program::pubkey::Pubkey) -> &mut Self {
        self.stake = Some(stake);
        self
    }
    /// `[optional account, default to
    /// 'SysvarRent111111111111111111111111111111111']` Rent sysvar
    #[inline(always)]
    pub fn rent(&mut self, rent: solana_program::pubkey::Pubkey) -> &mut Self {
        self.rent = Some(rent);
        self
    }
    #[inline(always)]
    pub fn staker(&mut self, staker: Pubkey) -> &mut Self {
        self.staker = Some(staker);
        self
    }
    #[inline(always)]
    pub fn withdrawer(&mut self, withdrawer: Pubkey) -> &mut Self {
        self.withdrawer = Some(withdrawer);
        self
    }
    #[inline(always)]
    pub fn unix_timestamp(&mut self, unix_timestamp: i64) -> &mut Self {
        self.unix_timestamp = Some(unix_timestamp);
        self
    }
    #[inline(always)]
    pub fn epoch(&mut self, epoch: u64) -> &mut Self {
        self.epoch = Some(epoch);
        self
    }
    #[inline(always)]
    pub fn custodian(&mut self, custodian: Pubkey) -> &mut Self {
        self.custodian = Some(custodian);
        self
    }
    /// Add an additional account to the instruction.
    #[inline(always)]
    pub fn add_remaining_account(
        &mut self,
        account: solana_program::instruction::AccountMeta,
    ) -> &mut Self {
        self.__remaining_accounts.push(account);
        self
    }
    /// Add additional accounts to the instruction.
    #[inline(always)]
    pub fn add_remaining_accounts(
        &mut self,
        accounts: &[solana_program::instruction::AccountMeta],
    ) -> &mut Self {
        self.__remaining_accounts.extend_from_slice(accounts);
        self
    }
    #[allow(clippy::clone_on_copy)]
    pub fn instruction(&self) -> solana_program::instruction::Instruction {
        let accounts = Initialize {
            stake: self.stake.expect("stake is not set"),
            rent: self.rent.unwrap_or(solana_program::pubkey!(
                "SysvarRent111111111111111111111111111111111"
            )),
        };
        let args = InitializeInstructionArgs {
            staker: self.staker.clone().expect("staker is not set"),
            withdrawer: self.withdrawer.clone().expect("withdrawer is not set"),
            unix_timestamp: self
                .unix_timestamp
                .clone()
                .expect("unix_timestamp is not set"),
            epoch: self.epoch.clone().expect("epoch is not set"),
            custodian: self.custodian.clone().expect("custodian is not set"),
        };

        accounts.instruction_with_remaining_accounts(args, &self.__remaining_accounts)
    }
}

/// `initialize` CPI accounts.
pub struct InitializeCpiAccounts<'a, 'b> {
    /// The stake account to initialize
    pub stake: &'b solana_program::account_info::AccountInfo<'a>,
    /// Rent sysvar
    pub rent: &'b solana_program::account_info::AccountInfo<'a>,
}

/// `initialize` CPI instruction.
pub struct InitializeCpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_program::account_info::AccountInfo<'a>,
    /// The stake account to initialize
    pub stake: &'b solana_program::account_info::AccountInfo<'a>,
    /// Rent sysvar
    pub rent: &'b solana_program::account_info::AccountInfo<'a>,
    /// The arguments for the instruction.
    pub __args: InitializeInstructionArgs,
}

impl<'a, 'b> InitializeCpi<'a, 'b> {
    pub fn new(
        program: &'b solana_program::account_info::AccountInfo<'a>,
        accounts: InitializeCpiAccounts<'a, 'b>,
        args: InitializeInstructionArgs,
    ) -> Self {
        Self {
            __program: program,
            stake: accounts.stake,
            rent: accounts.rent,
            __args: args,
        }
    }
    #[inline(always)]
    pub fn invoke(&self) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(&[], &[])
    }
    #[inline(always)]
    pub fn invoke_with_remaining_accounts(
        &self,
        remaining_accounts: &[(
            &'b solana_program::account_info::AccountInfo<'a>,
            bool,
            bool,
        )],
    ) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(&[], remaining_accounts)
    }
    #[inline(always)]
    pub fn invoke_signed(
        &self,
        signers_seeds: &[&[&[u8]]],
    ) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(signers_seeds, &[])
    }
    #[allow(clippy::clone_on_copy)]
    #[allow(clippy::vec_init_then_push)]
    pub fn invoke_signed_with_remaining_accounts(
        &self,
        signers_seeds: &[&[&[u8]]],
        remaining_accounts: &[(
            &'b solana_program::account_info::AccountInfo<'a>,
            bool,
            bool,
        )],
    ) -> solana_program::entrypoint::ProgramResult {
        let mut accounts = Vec::with_capacity(2 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.stake.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.rent.key,
            false,
        ));
        remaining_accounts.iter().for_each(|remaining_account| {
            accounts.push(solana_program::instruction::AccountMeta {
                pubkey: *remaining_account.0.key,
                is_signer: remaining_account.1,
                is_writable: remaining_account.2,
            })
        });
        let mut data = InitializeInstructionData::new().try_to_vec().unwrap();
        let mut args = self.__args.try_to_vec().unwrap();
        data.append(&mut args);

        let instruction = solana_program::instruction::Instruction {
            program_id: crate::STAKE_PROGRAM_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(3 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.stake.clone());
        account_infos.push(self.rent.clone());
        remaining_accounts
            .iter()
            .for_each(|remaining_account| account_infos.push(remaining_account.0.clone()));

        if signers_seeds.is_empty() {
            solana_program::program::invoke(&instruction, &account_infos)
        } else {
            solana_program::program::invoke_signed(&instruction, &account_infos, signers_seeds)
        }
    }
}

/// Instruction builder for `Initialize` via CPI.
///
/// ### Accounts:
///
///   0. `[writable]` stake
///   1. `[]` rent
#[derive(Clone, Debug)]
pub struct InitializeCpiBuilder<'a, 'b> {
    instruction: Box<InitializeCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> InitializeCpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(InitializeCpiBuilderInstruction {
            __program: program,
            stake: None,
            rent: None,
            staker: None,
            withdrawer: None,
            unix_timestamp: None,
            epoch: None,
            custodian: None,
            __remaining_accounts: Vec::new(),
        });
        Self { instruction }
    }
    /// The stake account to initialize
    #[inline(always)]
    pub fn stake(&mut self, stake: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.stake = Some(stake);
        self
    }
    /// Rent sysvar
    #[inline(always)]
    pub fn rent(&mut self, rent: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.rent = Some(rent);
        self
    }
    #[inline(always)]
    pub fn staker(&mut self, staker: Pubkey) -> &mut Self {
        self.instruction.staker = Some(staker);
        self
    }
    #[inline(always)]
    pub fn withdrawer(&mut self, withdrawer: Pubkey) -> &mut Self {
        self.instruction.withdrawer = Some(withdrawer);
        self
    }
    #[inline(always)]
    pub fn unix_timestamp(&mut self, unix_timestamp: i64) -> &mut Self {
        self.instruction.unix_timestamp = Some(unix_timestamp);
        self
    }
    #[inline(always)]
    pub fn epoch(&mut self, epoch: u64) -> &mut Self {
        self.instruction.epoch = Some(epoch);
        self
    }
    #[inline(always)]
    pub fn custodian(&mut self, custodian: Pubkey) -> &mut Self {
        self.instruction.custodian = Some(custodian);
        self
    }
    /// Add an additional account to the instruction.
    #[inline(always)]
    pub fn add_remaining_account(
        &mut self,
        account: &'b solana_program::account_info::AccountInfo<'a>,
        is_writable: bool,
        is_signer: bool,
    ) -> &mut Self {
        self.instruction
            .__remaining_accounts
            .push((account, is_writable, is_signer));
        self
    }
    /// Add additional accounts to the instruction.
    ///
    /// Each account is represented by a tuple of the `AccountInfo`, a `bool`
    /// indicating whether the account is writable or not, and a `bool`
    /// indicating whether the account is a signer or not.
    #[inline(always)]
    pub fn add_remaining_accounts(
        &mut self,
        accounts: &[(
            &'b solana_program::account_info::AccountInfo<'a>,
            bool,
            bool,
        )],
    ) -> &mut Self {
        self.instruction
            .__remaining_accounts
            .extend_from_slice(accounts);
        self
    }
    #[inline(always)]
    pub fn invoke(&self) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed(&[])
    }
    #[allow(clippy::clone_on_copy)]
    #[allow(clippy::vec_init_then_push)]
    pub fn invoke_signed(
        &self,
        signers_seeds: &[&[&[u8]]],
    ) -> solana_program::entrypoint::ProgramResult {
        let args = InitializeInstructionArgs {
            staker: self.instruction.staker.clone().expect("staker is not set"),
            withdrawer: self
                .instruction
                .withdrawer
                .clone()
                .expect("withdrawer is not set"),
            unix_timestamp: self
                .instruction
                .unix_timestamp
                .clone()
                .expect("unix_timestamp is not set"),
            epoch: self.instruction.epoch.clone().expect("epoch is not set"),
            custodian: self
                .instruction
                .custodian
                .clone()
                .expect("custodian is not set"),
        };
        let instruction = InitializeCpi {
            __program: self.instruction.__program,

            stake: self.instruction.stake.expect("stake is not set"),

            rent: self.instruction.rent.expect("rent is not set"),
            __args: args,
        };
        instruction.invoke_signed_with_remaining_accounts(
            signers_seeds,
            &self.instruction.__remaining_accounts,
        )
    }
}

#[derive(Clone, Debug)]
struct InitializeCpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_program::account_info::AccountInfo<'a>,
    stake: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    rent: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    staker: Option<Pubkey>,
    withdrawer: Option<Pubkey>,
    unix_timestamp: Option<i64>,
    epoch: Option<u64>,
    custodian: Option<Pubkey>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(
        &'b solana_program::account_info::AccountInfo<'a>,
        bool,
        bool,
    )>,
}
