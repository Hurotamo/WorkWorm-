use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
    system_instruction,
    program::{invoke, invoke_signed},
    clock::Clock,
    borsh::{BorshDeserialize, BorshSerialize},
};

// Constants for job posting and escrow
pub const JOB_POSTING_SEED: &[u8] = b"job_posting";
pub const ESCROW_SEED: &[u8] = b"escrow";

// Job status to track different stages
#[derive(Debug, PartialEq, Clone, BorshSerialize, BorshDeserialize)]
pub enum JobStatus {
    Posted,
    Accepted,
    InProgress,
    Completed,
    Cancelled,
}

// Structure for a milestone
#[derive(Debug, Clone, BorshSerialize, BorshDeserialize)]
pub struct Milestone {
    pub description: String,
    pub payment_amount: u64,
    pub completed: bool,
    pub deadline: Option<i64>, // New field for milestone deadline
}

// Structure for job rating
#[derive(Debug, Clone, BorshSerialize, BorshDeserialize)]
pub struct JobRating {
    pub employer: Pubkey,
    pub freelancer: Pubkey,
    pub rating: u8, // Scale of 1-5
    pub feedback: String,
}

// Structure for the job posting
#[derive(Debug, Clone, BorshSerialize, BorshDeserialize)]
pub struct JobPosting {
    pub employer: Pubkey,
    pub freelancer: Option<Pubkey>,
    pub status: JobStatus,
    pub milestones: Vec<Milestone>, // New field for milestones
    pub total_payment: u64,
    pub expiration_time: i64, // New field for job expiration time
}

// Entry point for the smart contract
entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let employer_account = next_account_info(accounts_iter)?;
    let escrow_account = next_account_info(accounts_iter)?;
    let job_account = next_account_info(accounts_iter)?; // New account for job posting

    // Decode instruction data to determine action (post job, accept job, etc.)
    let action = instruction_data[0]; // Simplified action decoder
    let job_amount = u64::from_le_bytes(instruction_data[1..9].try_into().unwrap());

    match action {
        1 => { // Post a job
            post_job(program_id, employer_account, escrow_account, job_account, job_amount)?;
        },
        2 => { // Accept the job
            accept_job(program_id, employer_account, job_account)?;
        },
        3 => { // Complete a milestone
            let milestone_index = instruction_data[9] as usize; // Simplified milestone index decoder
            complete_milestone(program_id, employer_account, job_account, milestone_index)?;
        },
        4 => { // Confirm job completion (escrow release condition)
            confirm_completion(program_id, employer_account, job_account)?;
        },
        5 => { // Dispute the job
            dispute_job(program_id, employer_account, job_account)?;
        },
        6 => { // Rate a job
            let rating = instruction_data[10]; // Assume rating is at byte index 10
            let feedback_length = instruction_data[11] as usize; // Length of feedback string
            let feedback = String::from_utf8(instruction_data[12..12 + feedback_length].to_vec()).unwrap();
            rate_job(program_id, employer_account, job_account, rating, feedback)?;
        },
        _ => {
            return Err(ProgramError::InvalidInstructionData);
        }
    }

    Ok(())
}

fn post_job(
    program_id: &Pubkey,
    employer_account: &AccountInfo,
    escrow_account: &AccountInfo,
    job_account: &AccountInfo, // New parameter for the job account
    total_payment: u64,
) -> ProgramResult {
    // Validate the accounts
    if employer_account.owner != program_id {
        return Err(ProgramError::IncorrectProgramId);
    }

    // Get the current time for expiration check
    let clock = Clock::get()?;
    let expiration_time = clock.unix_timestamp + 604800; // 1 week expiration

    // Transfer the funds to the escrow account
    let transfer_ix = system_instruction::transfer(
        employer_account.key,
        escrow_account.key,
        total_payment,
    );

    invoke(
        &transfer_ix,
        &[employer_account.clone(), escrow_account.clone()],
    )?;

    // Create and store the job posting data
    let job_posting = JobPosting {
        employer: *employer_account.key,
        freelancer: None,
        status: JobStatus::Posted,
        milestones: Vec::new(), // Initialize with no milestones
        total_payment,
        expiration_time,
    };

    // Serialize and store the job posting data in the job account
    job_posting.serialize(&mut &mut job_account.data.borrow_mut()[..])?;

    msg!("Job posted with escrow amount of {} lamports, expires at {}", total_payment, expiration_time);

    Ok(())
}

fn accept_job(
    program_id: &Pubkey,
    freelancer_account: &AccountInfo,
    job_account: &AccountInfo,
) -> ProgramResult {
    // Ensure the freelancer is valid
    if freelancer_account.owner != program_id {
        return Err(ProgramError::IncorrectProgramId);
    }

    // Retrieve and update the job posting data
    let mut job_posting = JobPosting::try_from_slice(&job_account.data.borrow())?;
    if job_posting.freelancer.is_some() {
        return Err(ProgramError::Custom(3)); // Custom error for job already accepted
    }
    
    job_posting.freelancer = Some(*freelancer_account.key);
    job_posting.status = JobStatus::Accepted;
    job_posting.serialize(&mut &mut job_account.data.borrow_mut()[..])?; // Update job posting in storage

    msg!("Job accepted by freelancer: {}", freelancer_account.key);

    Ok(())
}

fn complete_milestone(
    program_id: &Pubkey,
    employer_account: &AccountInfo,
    job_account: &AccountInfo,
    milestone_index: usize,
) -> ProgramResult {
    // Ensure the employer is the one completing the milestone
    if employer_account.owner != program_id {
        return Err(ProgramError::IncorrectProgramId);
    }

    // Retrieve the job posting data
    let mut job_posting = JobPosting::try_from_slice(&job_account.data.borrow())?;

    if milestone_index >= job_posting.milestones.len() {
        return Err(ProgramError::InvalidArgument); // Invalid milestone index
    }

    let milestone = &mut job_posting.milestones[milestone_index];

    // Check if the milestone deadline has passed
    let clock = Clock::get()?;
    if let Some(deadline) = milestone.deadline {
        if clock.unix_timestamp > deadline {
            return Err(ProgramError::Custom(2)); // Custom error for missed deadline
        }
    }

    // Mark milestone as completed
    milestone.completed = true;

    // Release funds for the completed milestone
    let transfer_ix = system_instruction::transfer(
        job_account.key, // Transfer from escrow
        employer_account.key,
        milestone.payment_amount,
    );

    invoke(
        &transfer_ix,
        &[job_account.clone(), employer_account.clone()],
    )?;

    msg!("Milestone completed: {}. Funds released: {} lamports", milestone.description, milestone.payment_amount);

    // Update job posting in storage
    job_posting.serialize(&mut &mut job_account.data.borrow_mut()[..])?;

    Ok(())
}

fn confirm_completion(
    program_id: &Pubkey,
    employer_account: &AccountInfo,
    job_account: &AccountInfo,
) -> ProgramResult {
    // Ensure the employer is valid
    if employer_account.owner != program_id {
        return Err(ProgramError::IncorrectProgramId);
    }

    // Logic to release funds if all milestones are completed
    // Check the milestones to see if they are all completed
    let mut job_posting = JobPosting::try_from_slice(&job_account.data.borrow())?;
    let all_milestones_completed = job_posting.milestones.iter().all(|m| m.completed);

    if !all_milestones_completed {
        return Err(ProgramError::Custom(1)); // Custom error for uncompleted milestones
    }

    // Release remaining funds from escrow to the freelancer
    let remaining_funds = job_posting.total_payment; // Assuming all funds remaining
    let transfer_ix = system_instruction::transfer(
        job_account.key,
        employer_account.key,
        remaining_funds,
    );

    invoke(
        &transfer_ix,
        &[job_account.clone(), employer_account.clone()],
    )?;

    msg!("All milestones completed. Remaining funds transferred to freelancer: {}", employer_account.key);

    // Mark job as completed
    job_posting.status = JobStatus::Completed;
    job_posting.serialize(&mut &mut job_account.data.borrow_mut()[..])?;

    Ok(())
}

// Function to dispute a job
fn dispute_job(
    _program_id: &Pubkey,
    _employer_account: &AccountInfo,
    _job_account: &AccountInfo,
) -> ProgramResult {
    // Placeholder logic for dispute resolution
    msg!("Dispute initiated for job.");
    Ok(())
}

// Function to rate a job
fn rate_job(
    program_id: &Pubkey,
    employer_account: &AccountInfo,
    job_account: &AccountInfo,
    rating: u8,
    feedback: String,
) -> ProgramResult {
    // Ensure the employer is valid
    if employer_account.owner != program_id {
        return Err(ProgramError::IncorrectProgramId);
    }

    // Validate rating value
    if rating < 1 || rating > 5 {
        return Err(ProgramError::InvalidArgument); // Rating must be between 1-5
    }

    // Retrieve the job posting data
    let job_posting = JobPosting::try_from_slice(&job_account.data.borrow())?;

    // Create a new job rating
    let job_rating = JobRating {
        employer: *employer_account.key,
        freelancer: job_posting.freelancer.ok_or(ProgramError::InvalidArgument)?,
        rating,
        feedback,
    };

    // (Optional) Store job rating in a separate account or update existing one
    msg!("Job rated. Rating: {}. Feedback: {}", rating, feedback);

    Ok(())
}

// Function to get job posting
fn get_job_posting(job_account: &AccountInfo) -> ProgramResult {
    let job_posting = JobPosting::try_from_slice(&job_account.data.borrow())?;
    msg!("Job Posting: {:?}", job_posting);
    Ok(())
}
