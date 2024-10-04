import {
  Connection,
  PublicKey,
  Keypair,
  SystemProgram,
  Transaction,
  sendAndConfirmTransaction,
} from "@solana/web3.js";
import * as borsh from "borsh";

// Borsh schema definition for JobStatus, Milestone, JobRating, and JobPosting
enum JobStatus {
  Posted = 0,
  Accepted,
  InProgress,
  Completed,
  Cancelled,
}

class Milestone {
  description: string;
  paymentAmount: number;
  completed: boolean;
  deadline: number | null;

  constructor(fields: { description: string; paymentAmount: number; completed: boolean; deadline: number | null }) {
    this.description = fields.description;
    this.paymentAmount = fields.paymentAmount;
    this.completed = fields.completed;
    this.deadline = fields.deadline;
  }
}

class JobRating {
  employer: PublicKey;
  freelancer: PublicKey;
  rating: number;
  feedback: string;

  constructor(fields: { employer: PublicKey; freelancer: PublicKey; rating: number; feedback: string }) {
    this.employer = fields.employer;
    this.freelancer = fields.freelancer;
    this.rating = fields.rating;
    this.feedback = fields.feedback;
  }
}

class JobPosting {
  employer: PublicKey;
  freelancer: PublicKey | null;
  status: JobStatus;
  milestones: Milestone[];
  totalPayment: number;
  expirationTime: number;

  constructor(fields: {
    employer: PublicKey;
    freelancer: PublicKey | null;
    status: JobStatus;
    milestones: Milestone[];
    totalPayment: number;
    expirationTime: number;
  }) {
    this.employer = fields.employer;
    this.freelancer = fields.freelancer;
    this.status = fields.status;
    this.milestones = fields.milestones;
    this.totalPayment = fields.totalPayment;
    this.expirationTime = fields.expirationTime;
  }
}

// Borsh schema
const MilestoneSchema = new Map([
  [
    Milestone,
    {
      kind: "struct",
      fields: [
        ["description", "string"],
        ["paymentAmount", "u64"],
        ["completed", "bool"],
        ["deadline", { kind: "option", type: "i64" }],
      ],
    },
  ],
]);

const JobPostingSchema = new Map([
  [
    JobPosting,
    {
      kind: "struct",
      fields: [
        ["employer", [32]], // PublicKey is 32 bytes
        ["freelancer", { kind: "option", type: [32] }], // Optional PublicKey
        ["status", "u8"], // JobStatus as u8
        ["milestones", { kind: "vec", type: Milestone }], // Array of Milestone
        ["totalPayment", "u64"], // Total payment in u64
        ["expirationTime", "i64"], // Expiration time in i64
      ],
    },
  ],
]);

// Helper function to serialize job posting data
function serializeJobPosting(jobPosting: JobPosting): Uint8Array {
  return borsh.serialize(JobPostingSchema, jobPosting);
}

// Helper function to deserialize job posting data
function deserializeJobPosting(buffer: Buffer): JobPosting {
  return borsh.deserialize(JobPostingSchema, JobPosting, buffer);
}

// Example function to post a new job
export async function postJob(
  connection: Connection,
  programId: PublicKey,
  employer: Keypair,
  escrowAccount: Keypair,
  jobPosting: JobPosting
) {
  const jobAccount = Keypair.generate();

  const instructionData = Buffer.concat([
    Buffer.from([1]), // Action 1: Post Job
    serializeJobPosting(jobPosting),
  ]);

  const transaction = new Transaction().add({
    keys: [
      { pubkey: employer.publicKey, isSigner: true, isWritable: true },
      { pubkey: escrowAccount.publicKey, isSigner: true, isWritable: true },
      { pubkey: jobAccount.publicKey, isSigner: true, isWritable: true },
    ],
    programId,
    data: instructionData,
  });

  const signature = await sendAndConfirmTransaction(
    connection,
    transaction,
    [employer, escrowAccount, jobAccount]
  );
  console.log("Job posted successfully:", signature);
}

// Example function to accept a job
export async function acceptJob(
  connection: Connection,
  programId: PublicKey,
  employer: PublicKey,
  jobAccount: PublicKey,
  freelancer: Keypair
) {
  const instructionData = Buffer.from([2]); // Action 2: Accept Job

  const transaction = new Transaction().add({
    keys: [
      { pubkey: freelancer.publicKey, isSigner: true, isWritable: true },
      { pubkey: employer, isSigner: false, isWritable: true },
      { pubkey: jobAccount, isSigner: false, isWritable: true },
    ],
    programId,
    data: instructionData,
  });

  const signature = await sendAndConfirmTransaction(connection, transaction, [freelancer]);
  console.log("Job accepted successfully:", signature);
}

// Example function to complete a milestone
export async function completeMilestone(
  connection: Connection,
  programId: PublicKey,
  employer: PublicKey,
  jobAccount: PublicKey,
  milestoneIndex: number,
  freelancer: Keypair
) {
  const instructionData = Buffer.from([3, milestoneIndex]); // Action 3: Complete Milestone

  const transaction = new Transaction().add({
    keys: [
      { pubkey: freelancer.publicKey, isSigner: true, isWritable: true },
      { pubkey: employer, isSigner: false, isWritable: true },
      { pubkey: jobAccount, isSigner: false, isWritable: true },
    ],
    programId,
    data: instructionData,
  });

  const signature = await sendAndConfirmTransaction(connection, transaction, [freelancer]);
  console.log("Milestone completed successfully:", signature);
}

// Add similar functions for `confirmCompletion`, `disputeJob`, and `rateJob`

// Example usage
(async () => {
  const connection = new Connection("https://api.mainnet-beta.solana.com", "confirmed");
  const programId = new PublicKey("YOUR_PROGRAM_ID");

  const employer = Keypair.generate(); // Replace with your keypair
  const escrowAccount = Keypair.generate(); // Replace with your keypair
  const jobPosting = new JobPosting({
    employer: employer.publicKey,
    freelancer: null,
    status: JobStatus.Posted,
    milestones: [],
    totalPayment: 1000,
    expirationTime: Date.now() + 60 * 60 * 24 * 7, // One week expiration
  });

  await postJob(connection, programId, employer, escrowAccount, jobPosting);
})();
