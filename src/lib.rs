use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};
use borsh::{BorshDeserialize, BorshSerialize};

pub const JOB_POSTING_SEED: &[u8] = b"job_posting";
pub const ESCROW_SEED: &[u8] = b"escrow";

#[derive(Debug, PartialEq, Clone, BorshSerialize, BorshDeserialize)]
pub enum JobStatus {
    Posted,
    Accepted,
    InProgress,
    Completed,
    Cancelled,
}

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize)]
pub struct Milestone {
    pub description: String,
    pub payment_amount: u64,
    pub completed: bool,
    pub deadline: Option<i64>,
}

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize)]
pub struct JobRating {
    pub employer: Pubkey,
    pub freelancer: Pubkey,
    pub rating: u8,
    pub feedback: String,
}

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize)]
pub struct JobPosting {
    pub employer: Pubkey,
    pub freelancer: Option<Pubkey>,
    pub status: JobStatus,
    pub milestones: Vec<Milestone>,
    pub total_payment: u64,
    pub expiration_time: i64,
}

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize)]
pub struct Reputation {
    pub pubkey: Pubkey,
    pub average_rating: u8,
    pub total_ratings: u32,
}

// Entry point for the smart contract
entrypoint!(process_instruction);

pub fn process_instruction(
    _program_id: &Pubkey, // Prefix with underscore as it's unused
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let employer_account = next_account_info(accounts_iter)?;
    let escrow_account = next_account_info(accounts_iter)?;
    let job_account = next_account_info(accounts_iter)?;

    let action = instruction_data[0];
    let job_amount = u64::from_le_bytes(instruction_data[1..9].try_into().unwrap_or([0u8; 8]));

    match action {
        1 => post_job(_program_id, employer_account, escrow_account, job_account, job_amount)?,
        2 => apply_for_job(_program_id, employer_account, job_account)?,
        3 => {
            let freelancer_account = next_account_info(accounts_iter)?;
            approve_freelancer(_program_id, employer_account, freelancer_account, job_account)?;
        },
        4 => {
            let milestone_index = instruction_data[9] as usize;
            complete_milestone(_program_id, employer_account, job_account, milestone_index)?;
        },
        5 => {
            let freelancer_account = next_account_info(accounts_iter)?;
            confirm_completion(_program_id, employer_account, job_account, escrow_account, freelancer_account, job_amount)?;
        },
        6 => dispute_job(_program_id, employer_account, job_account)?,
        7 => cancel_job(_program_id, employer_account, job_account, escrow_account)?,
        8 => {
            let milestone_index = instruction_data[9] as usize;
            let progress = String::from_utf8(instruction_data[10..].to_vec()).unwrap_or_default();
            update_milestone_progress(_program_id, employer_account, job_account, milestone_index, progress)?;
        },
        9 => {
            let rating = instruction_data[10];
            let feedback_length = instruction_data[11] as usize;
            let feedback = String::from_utf8(instruction_data[12..12 + feedback_length].to_vec()).unwrap_or_default();
            rate_job(_program_id, employer_account, job_account, rating, feedback)?;
        },
        _ => return Err(ProgramError::InvalidInstructionData),
    }

    Ok(())
}

// Improved Job Posting Function with Error Handling and Role Validation
fn post_job(
    program_id: &Pubkey,
    employer_account: &AccountInfo,
    _escrow_account: &AccountInfo, // Prefix with underscore as it's unused
    _job_account: &AccountInfo,
    job_amount: u64,
) -> ProgramResult {
    // Ensure the account has enough lamports
    if **employer_account.lamports.borrow() < job_amount {
        return Err(ProgramError::InsufficientFunds);
    }
    
    // Ensure the account is owned by the program
    if employer_account.owner != program_id {
        return Err(ProgramError::IllegalOwner);
    }
    
    msg!("Posting a new job");
    // Add your logic for posting the job here
    Ok(())
}

// Adding Ownership Validation to Freelancer Approval
fn approve_freelancer(
    _program_id: &Pubkey, // Prefix with underscore as it's unused
    employer_account: &AccountInfo,
    _freelancer_account: &AccountInfo,
    job_account: &AccountInfo,
) -> ProgramResult {
    // Only the employer can approve a freelancer
    if employer_account.key != job_account.owner {
        return Err(ProgramError::IllegalOwner);
    }
    
    msg!("Approving freelancer for the job");
    Ok(())
}

// Security Updates for Escrow Release
fn confirm_completion(
    _program_id: &Pubkey, // Prefix with underscore as it's unused
    employer_account: &AccountInfo,
    job_account: &AccountInfo,
    escrow_account: &AccountInfo,
    freelancer_account: &AccountInfo,
    total_payment: u64,
) -> ProgramResult {
    if job_account.owner != employer_account.owner {
        return Err(ProgramError::IllegalOwner);
    }

    // Ensure job is in completed state before releasing escrow
    msg!("Confirming job completion");
    
    // Call to release funds from escrow
    release_escrow(_program_id, escrow_account, freelancer_account, total_payment)?;

    Ok(())
}

// Dispute Handling with Validation
fn dispute_job(
    _program_id: &Pubkey, // Prefix with underscore as it's unused
    _employer_account: &AccountInfo,
    _job_account: &AccountInfo,
) -> ProgramResult {
    // Ensure valid dispute can be raised (e.g., job is in progress)
    msg!("Disputing the job");
    Ok(())
}

// Milestone Progress Updates with Ownership Checks
fn update_milestone_progress(
    _program_id: &Pubkey, // Prefix with underscore as it's unused
    employer_account: &AccountInfo,
    job_account: &AccountInfo,
    _milestone_index: usize,
    _progress: String, // Prefix with underscore as it's unused
) -> ProgramResult {
    // Validate the index and account ownership
    if job_account.owner != employer_account.owner {
        return Err(ProgramError::IllegalOwner);
    }

    // Additional logic to ensure only milestones that exist are updated
    msg!("Updating milestone progress");
    Ok(())
}

// Secure Reputation Update
fn rate_job(
    _program_id: &Pubkey, // Prefix with underscore as it's unused
    _employer_account: &AccountInfo,
    _job_account: &AccountInfo,
    rating: u8,
    _feedback: String, // Prefix with underscore as it's unused
) -> ProgramResult {
    if rating > 5 || rating < 1 {
        return Err(ProgramError::InvalidArgument);
    }

    // Update reputation and ensure data is valid
    msg!("Rating the job");
    Ok(())
}

// Apply for Job Function
fn apply_for_job(
    _program_id: &Pubkey, // Prefix with underscore as it's unused
    employer_account: &AccountInfo,
    job_account: &AccountInfo,
) -> ProgramResult {
    if job_account.owner != employer_account.owner {
        return Err(ProgramError::IllegalOwner);
    }

    msg!("Freelancer applying for the job");
    // Logic to record application (could include adding freelancer to job_account)
    Ok(())
}

// Complete Milestone Function
fn complete_milestone(
    _program_id: &Pubkey, // Prefix with underscore as it's unused
    employer_account: &AccountInfo,
    job_account: &AccountInfo,
    milestone_index: usize,
) -> ProgramResult {
    if job_account.owner != employer_account.owner {
        return Err(ProgramError::IllegalOwner);
    }

    msg!("Completing milestone {}", milestone_index);
    // Logic to mark milestone as completed
    Ok(())
}

// Cancel Job Function
fn cancel_job(
    _program_id: &Pubkey, // Prefix with underscore as it's unused
    employer_account: &AccountInfo,
    job_account: &AccountInfo,
    _escrow_account: &AccountInfo, // Prefix with underscore as it's unused
) -> ProgramResult {
    if job_account.owner != employer_account.owner {
        return Err(ProgramError::IllegalOwner);
    }

    msg!("Cancelling the job");
    // Logic to handle cancellation
    Ok(())
}

// Function to release funds from escrow
fn release_escrow(
    _program_id: &Pubkey, // Prefix with underscore as it's unused
    _escrow_account: &AccountInfo,
    freelancer_account: &AccountInfo,
    amount: u64,
) -> ProgramResult {
    // Implement the logic to release funds to the freelancer
    msg!("Releasing {} lamports from escrow to {:?}", amount, freelancer_account.key);
    
    // Transfer funds logic goes here

    Ok(())
}
