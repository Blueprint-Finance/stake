//! This code was AUTOGENERATED using the codama library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun codama to update it.
//!
//! <https://github.com/codama-idl/codama>

use borsh::{BorshDeserialize, BorshSerialize};

/// Accounts.
pub struct Split {
    /// The stake account to split. Must be in the Initialized or Stake state
    pub from: solana_program::pubkey::Pubkey,
    /// The uninitialized stake account to split to. Must be rent-exempt
    /// starting from solana 1.17.
    pub to: solana_program::pubkey::Pubkey,
    /// from's stake authority
    pub stake_authority: solana_program::pubkey::Pubkey,
}

impl Split {
    pub fn instruction(
        &self,
        args: SplitInstructionArgs,
    ) -> solana_program::instruction::Instruction {
        self.instruction_with_remaining_accounts(args, &[])
    }
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        args: SplitInstructionArgs,
        remaining_accounts: &[solana_program::instruction::AccountMeta],
    ) -> solana_program::instruction::Instruction {
        let mut accounts = Vec::with_capacity(3 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.from, false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.to, false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.stake_authority,
            true,
        ));
        accounts.extend_from_slice(remaining_accounts);
        let mut data = SplitInstructionData::new().try_to_vec().unwrap();
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
pub struct SplitInstructionData {
    discriminator: [u8; 8],
}

impl SplitInstructionData {
    pub fn new() -> Self {
        Self {
            discriminator: [124, 189, 27, 43, 216, 40, 147, 66],
        }
    }
}

impl Default for SplitInstructionData {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SplitInstructionArgs {
    pub lamports: u64,
}

/// Instruction builder for `Split`.
///
/// ### Accounts:
///
///   0. `[writable]` from
///   1. `[writable]` to
///   2. `[signer]` stake_authority
#[derive(Clone, Debug, Default)]
pub struct SplitBuilder {
    from: Option<solana_program::pubkey::Pubkey>,
    to: Option<solana_program::pubkey::Pubkey>,
    stake_authority: Option<solana_program::pubkey::Pubkey>,
    lamports: Option<u64>,
    __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl SplitBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    /// The stake account to split. Must be in the Initialized or Stake state
    #[inline(always)]
    pub fn from(&mut self, from: solana_program::pubkey::Pubkey) -> &mut Self {
        self.from = Some(from);
        self
    }
    /// The uninitialized stake account to split to. Must be rent-exempt
    /// starting from solana 1.17.
    #[inline(always)]
    pub fn to(&mut self, to: solana_program::pubkey::Pubkey) -> &mut Self {
        self.to = Some(to);
        self
    }
    /// from's stake authority
    #[inline(always)]
    pub fn stake_authority(
        &mut self,
        stake_authority: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.stake_authority = Some(stake_authority);
        self
    }
    #[inline(always)]
    pub fn lamports(&mut self, lamports: u64) -> &mut Self {
        self.lamports = Some(lamports);
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
        let accounts = Split {
            from: self.from.expect("from is not set"),
            to: self.to.expect("to is not set"),
            stake_authority: self.stake_authority.expect("stake_authority is not set"),
        };
        let args = SplitInstructionArgs {
            lamports: self.lamports.clone().expect("lamports is not set"),
        };

        accounts.instruction_with_remaining_accounts(args, &self.__remaining_accounts)
    }
}

/// `split` CPI accounts.
pub struct SplitCpiAccounts<'a, 'b> {
    /// The stake account to split. Must be in the Initialized or Stake state
    pub from: &'b solana_program::account_info::AccountInfo<'a>,
    /// The uninitialized stake account to split to. Must be rent-exempt
    /// starting from solana 1.17.
    pub to: &'b solana_program::account_info::AccountInfo<'a>,
    /// from's stake authority
    pub stake_authority: &'b solana_program::account_info::AccountInfo<'a>,
}

/// `split` CPI instruction.
pub struct SplitCpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_program::account_info::AccountInfo<'a>,
    /// The stake account to split. Must be in the Initialized or Stake state
    pub from: &'b solana_program::account_info::AccountInfo<'a>,
    /// The uninitialized stake account to split to. Must be rent-exempt
    /// starting from solana 1.17.
    pub to: &'b solana_program::account_info::AccountInfo<'a>,
    /// from's stake authority
    pub stake_authority: &'b solana_program::account_info::AccountInfo<'a>,
    /// The arguments for the instruction.
    pub __args: SplitInstructionArgs,
}

impl<'a, 'b> SplitCpi<'a, 'b> {
    pub fn new(
        program: &'b solana_program::account_info::AccountInfo<'a>,
        accounts: SplitCpiAccounts<'a, 'b>,
        args: SplitInstructionArgs,
    ) -> Self {
        Self {
            __program: program,
            from: accounts.from,
            to: accounts.to,
            stake_authority: accounts.stake_authority,
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
        let mut accounts = Vec::with_capacity(3 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.from.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.to.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.stake_authority.key,
            true,
        ));
        remaining_accounts.iter().for_each(|remaining_account| {
            accounts.push(solana_program::instruction::AccountMeta {
                pubkey: *remaining_account.0.key,
                is_signer: remaining_account.1,
                is_writable: remaining_account.2,
            })
        });
        let mut data = SplitInstructionData::new().try_to_vec().unwrap();
        let mut args = self.__args.try_to_vec().unwrap();
        data.append(&mut args);

        let instruction = solana_program::instruction::Instruction {
            program_id: crate::STAKE_PROGRAM_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(4 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.from.clone());
        account_infos.push(self.to.clone());
        account_infos.push(self.stake_authority.clone());
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

/// Instruction builder for `Split` via CPI.
///
/// ### Accounts:
///
///   0. `[writable]` from
///   1. `[writable]` to
///   2. `[signer]` stake_authority
#[derive(Clone, Debug)]
pub struct SplitCpiBuilder<'a, 'b> {
    instruction: Box<SplitCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> SplitCpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(SplitCpiBuilderInstruction {
            __program: program,
            from: None,
            to: None,
            stake_authority: None,
            lamports: None,
            __remaining_accounts: Vec::new(),
        });
        Self { instruction }
    }
    /// The stake account to split. Must be in the Initialized or Stake state
    #[inline(always)]
    pub fn from(&mut self, from: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.from = Some(from);
        self
    }
    /// The uninitialized stake account to split to. Must be rent-exempt
    /// starting from solana 1.17.
    #[inline(always)]
    pub fn to(&mut self, to: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.to = Some(to);
        self
    }
    /// from's stake authority
    #[inline(always)]
    pub fn stake_authority(
        &mut self,
        stake_authority: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.stake_authority = Some(stake_authority);
        self
    }
    #[inline(always)]
    pub fn lamports(&mut self, lamports: u64) -> &mut Self {
        self.instruction.lamports = Some(lamports);
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
        let args = SplitInstructionArgs {
            lamports: self
                .instruction
                .lamports
                .clone()
                .expect("lamports is not set"),
        };
        let instruction = SplitCpi {
            __program: self.instruction.__program,

            from: self.instruction.from.expect("from is not set"),

            to: self.instruction.to.expect("to is not set"),

            stake_authority: self
                .instruction
                .stake_authority
                .expect("stake_authority is not set"),
            __args: args,
        };
        instruction.invoke_signed_with_remaining_accounts(
            signers_seeds,
            &self.instruction.__remaining_accounts,
        )
    }
}

#[derive(Clone, Debug)]
struct SplitCpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_program::account_info::AccountInfo<'a>,
    from: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    to: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    stake_authority: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    lamports: Option<u64>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(
        &'b solana_program::account_info::AccountInfo<'a>,
        bool,
        bool,
    )>,
}
