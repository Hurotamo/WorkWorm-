import { Connection, PublicKey, clusterApiUrl, SystemProgram, Transaction, Keypair, sendAndConfirmTransaction } from '@solana/web3.js';

// Replace with your deployed program ID
const PROGRAM_ID = new PublicKey('YourDeployedProgramID');

// Initialize connection to the Solana devnet
const connection = new Connection(clusterApiUrl('devnet'), 'confirmed');

// Function to post a job
async function postJob(employerPublicKey, jobAmount) {
    const employerKeypair = Keypair.fromSecretKey(employerPublicKey); // Get employer's keypair

    const transaction = new Transaction().add(
        // Create an instruction for posting a job
        SystemProgram.createAccount({
            fromPubkey: employerKeypair.publicKey,
            newAccountPubkey: employerKeypair.publicKey, // Use the job account address
            lamports: await connection.getMinimumBalanceForRentExemption(1024),
            space: 1024,
            programId: PROGRAM_ID,
        }),
        // Add your custom program instruction here for posting a job
        {
            keys: [
                { pubkey: employerKeypair.publicKey, isSigner: true, isWritable: true },
                // Add other accounts like escrow and job account
            ],
            programId: PROGRAM_ID,
            data: Buffer.from([1, ...new Uint64Array([jobAmount]).buffer]), // Action for posting a job
        }
    );

    // Send and confirm the transaction
    await sendAndConfirmTransaction(connection, transaction, [employerKeypair]);
}

// Function to accept a job
async function acceptJob(employerPublicKey) {
    // Similar to postJob, create a transaction for accepting a job
}

// Function to complete a milestone
async function completeMilestone(milestoneIndex) {
    // Similar to postJob, create a transaction for completing a milestone
}

// Event listeners for UI elements
document.getElementById('connectWallet').addEventListener('click', async () => {
    if (window.ethereum) {
        // Connect to the user's wallet and get their public key
        const accounts = await window.ethereum.request({ method: 'eth_requestAccounts' });
        const employerPublicKey = new PublicKey(accounts[0]); // Use the account as the public key

        // Call postJob or other functions as needed
        const jobAmount = 1000; // Example amount
        await postJob(employerPublicKey, jobAmount);
    } else {
        alert('Please install a Solana wallet.');
    }
});
