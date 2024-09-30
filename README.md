# WorkWorm: Decentralized Freelance Marketplace

**WorkWorm** is a decentralized freelance marketplace built on the **Solana blockchain**, leveraging the **Wormhole Protocol** for cross-chain transactions. This platform allows freelancers and employers to engage in secure and trustless job transactions. Employers can post jobs, set milestones, and manage payments through an escrow system, while freelancers can accept jobs, complete milestones, and get paid directly through the Solana blockchain.

## Features

- **Job Posting**: Employers can post jobs with a total payment and expiration time.
- **Milestones**: Jobs can include multiple milestones, with payments tied to the completion of each milestone.
- **Escrow System**: Payments are locked in an escrow account until job completion or milestone fulfillment.
- **Job Acceptance**: Freelancers can accept posted jobs and start working on them.
- **Completion Verification**: Employers verify the completion of milestones, and payments are released accordingly.
- **Job Ratings**: Both employers and freelancers can rate and provide feedback after the job is completed.
- **Cross-chain Functionality**: Leveraging Wormhole Protocol for transactions across different blockchains.

## Smart Contract Overview

### Modules

- **Job Posting**: Employers can create job postings with associated payments and milestones.
- **Milestone Management**: Employers and freelancers can track and complete milestones, ensuring secure payments for completed work.
- **Escrow System**: Ensures funds are locked until job completion, providing security to both parties.
- **Job Rating**: A rating system for employers and freelancers to assess job quality and experience.

### Core Structures

- **Job Status**: Enum to track job progression (`Posted`, `Accepted`, `InProgress`, `Completed`, `Cancelled`).
- **Milestone**: Tracks the description, payment amount, and completion status of individual job milestones.
- **Job Posting**: Main structure that holds the employer, freelancer, milestones, payment info, and job status.
- **Job Rating**: Allows employers and freelancers to provide feedback and rate each other based on job performance.

### Key Functions

- **Post Job**: Creates a job posting, locks funds in escrow, and sets an expiration time.
- **Accept Job**: Allows freelancers to accept a job and start working on it.
- **Complete Milestone**: Marks a milestone as completed and releases the corresponding payment.
- **Confirm Completion**: Confirms all milestones are completed and releases the remaining escrow funds to the freelancer.
- **Dispute Job**: Placeholder logic for initiating a job dispute.
- **Rate Job**: Allows employers to rate and provide feedback on freelancers after job completion.

### Escrow System

All payments for jobs are handled through an escrow system, ensuring that funds are securely locked during the course of the job and only released upon milestone completion or job dispute resolution. This adds an extra layer of security for both freelancers and employers.

## Installation and Deployment

### Prerequisites

- Rust programming language
- Solana CLI installed and configured
- Anchor framework (optional for easier development)
- Wormhole SDK for cross-chain operations

### Build and Deploy

1. **Clone the repository**:
   ```bash
   git clone https://github.com/Hurotamo/workworm-
   cd workworm-
   ```

2. **Build the smart contract**:
   ```bash
   cargo build-bpf --manifest-path=path-to-cargo-toml --bpf-out-dir=target/deploy
   ```

3. **Deploy the smart contract**:
   Use the Solana CLI or an Anchor-based deployment script to deploy the contract to the Solana blockchain.

   ```bash
   solana program deploy target/deploy/WorkWorm-.so
   ```

### Interact with the Program

You can interact with the deployed smart contract using custom scripts or integrate it into a decentralized application (dApp) frontend, supporting wallet connectivity for Phantom or Solflare.

## Cross-Chain Capabilities

WorkWorm uses the **Wormhole Protocol** to enable cross-chain interactions, allowing users to engage with the platform from various blockchain networks. This enables secure and efficient job postings, payments, and milestone tracking across different blockchains.

## License

This project is licensed under the MIT License.

## Contributing

Contributions are welcome! Please submit a pull request or raise an issue on GitHub for any suggestions or bug reports.
