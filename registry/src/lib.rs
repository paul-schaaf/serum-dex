#![cfg_attr(feature = "strict", deny(warnings))]
#![allow(dead_code)]

use borsh::{BorshDeserialize, BorshSchema, BorshSerialize};
use serum_common::pack::*;
use solana_client_gen::prelude::*;

pub mod access_control;
pub mod accounts;
pub mod error;

#[cfg_attr(feature = "client", solana_client_gen)]
pub mod instruction {
    use super::*;
    #[derive(Debug, BorshSerialize, BorshDeserialize, BorshSchema)]
    pub enum RegistryInstruction {
        /// Initializes the registry instance for use.
        ///
        /// Accounts:
        ///
        /// 0. `[writable]` Registrar to initialize.
        /// 1. `[]`         SRM "stake-intent" vault.
        /// 2. `[]`         MSRM "stake-intent" vault.
        /// 3. `[]`         Rent sysvar.
        Initialize {
            /// The priviledged account.
            authority: Pubkey,
            /// Nonce for deriving the vault authority address.
            nonce: u8,
            /// Number of seconds that must pass for a withdrawal to complete.
            withdrawal_timelock: i64,
            /// Number of seconds after which an Entity becomes "deactivated".
            deactivation_timelock: i64,
            /// The amount of tokens that must be staked for an entity to be
            /// eligible for rewards.
            reward_activation_threshold: u64,
        },
        /// CreateEntity initializes the new "node" with the Registry,
        /// designated "inactive".
        ///
        /// Accounts:
        ///
        /// 0. `[writable]` Entity account.
        /// 1. `[signer]`   Leader of the node.
        /// 2. `[]`         Registrar.
        /// 3. `[]`         Rent sysvar.
        CreateEntity,
        /// UpdateEntity updates the leader of the node entity.
        ///
        /// Accounts:
        ///
        /// 0. `[writable]` Entity account.
        /// 1. `[signer]`   Leader of the entity.
        /// 2. `[]`         Registrar.
        UpdateEntity { leader: Pubkey },
        /// Joins the entity by creating a membership account.
        ///
        /// Accounts:
        ///
        /// 0. `[writable]` Member account being created.
        /// 1. `[]`         Entity to join.
        /// 2. `[]`         Registrar.
        /// 3. `[]`         Rent sysvar.
        CreateMember {
            /// An account that can withdrawal or stake on the beneficiary's
            /// behalf.
            delegate: Pubkey,
            /// Watchtower authority assigned to the resulting member account.
            watchtower: accounts::Watchtower,
        },
        /// Accounts:
        ///
        /// 0. `[writable]` Member account.
        /// 1. `[signed]`   Beneficiary of the member account.
        UpdateMember {
            watchtower: Option<accounts::Watchtower>,
            /// Delegate can only be updated if the delegate's book balance is 0.
            delegate: Option<Pubkey>,
        },
        /// Accounts:
        ///
        /// 0. `[writable]` Member account.
        /// 1. `[signed]`   Beneficiary of the member account.
        /// 2. `[]`         Registrar.
        /// 3. `[writable]` Current entity of the member.
        /// 4. `[writable]` New entity of the member.
        /// 5. `[]`         Clock sysvar.
        /// ..              Pool accounts. SRM pool must be before MSRM pool.
        SwitchEntity,
        /// Accounts:
        ///
        /// Lockup whitelist relay account interface:
        ///
        /// 0. `[]`          Member account's delegate, e.g., the lockup's
        ///                  program-derived-adddress. If not a delegated
        ///                  instruction, then a dummy account.
        /// 1. `[writable]`  The depositing token account (sender).
        /// 2. `[writable]`  Vault (receiver).
        /// 3. `[]/[signer]` Delegate/owner of the depositing token account.
        ///                  If delegate, then the vault authority's
        ///                  program-derived address.
        /// 4. `[]`          SPL token program.
        ///
        /// Program specific.
        ///
        /// 5. `[writable]` Member account responsibile for the stake.
        /// 6. `[signer]`   Beneficiary of the Member account being staked.
        /// 7. `[writable]` Entity account to stake to.
        /// 8. `[]`         Registrar.
        /// 9. `[]`         Clock.
        Deposit { amount: u64 },
        /// Accounts:
        ///
        /// Same as StakeIntent.
        Withdraw { amount: u64 },
        /// Accounts:
        ///
        /// Same as StakeIntent, substituting Accounts[1] for the pool's vault.
        ///
        Stake { amount: u64 },
        /// Initiates a stake withdrawal. Funds are locked up until the
        /// withdrawl timelock passes.
        ///
        /// Accounts:
        ///
        /// 0. `[writable]  PendingWithdrawal account to initialize.
        /// 1  `[signed]`   Benficiary of the Stake account.
        /// 2. `[writable]` The Member account to withdraw from.
        /// 3. `[writable]` Entity the Stake is associated with.
        /// 4. `[writable]` Registrar.
        /// 5. `[writable]` SRM escrow vault.
        /// 6. `[writable]` MSRM escrow vault.
        /// 7. `[]`         Registrar vault authority.
        /// 8. `[]`         Token program.
        /// 9. `[]`         Rent sysvar.
        /// 10. `[]`        Clock sysvar.
        ///
        /// ..              Pool accounts.
        ///
        /// Delegate only.
        ///
        /// 7. `[signed]?`  Delegate owner of the Member account.
        StartStakeWithdrawal { amount: u64 },
        /// Completes the pending withdrawal once the timelock period passes.
        ///
        /// Accounts:
        ///
        /// 0. `[writable]  PendingWithdrawal account to complete.
        /// 1. `[signed]`   Beneficiary/delegate of the member account.
        /// 2. `[writable]` Member account to withdraw from.
        /// 3. `[writable]` Entity account the member is associated with.
        /// 4. `[]`         SPL token program (SRM).
        /// 5. `[]`         SPL mega token program (MSRM).
        /// 6. `[writable]` SRM token account to send to upon redemption
        /// 7. `[writable]` MSRM token account to send to upon redemption
        EndStakeWithdrawal,
    }
}

serum_common::packable!(instruction::RegistryInstruction);
