import { useState, useEffect } from 'react';
import { Connection, PublicKey } from '@solana/web3.js';

function WalletBalance({ account }) {
  const [balance, setBalance] = useState(null);

  useEffect(() => {
    const loadBalance = async () => {
      if (account) {
        const connection = new Connection('https://api.mainnet-beta.solana.com');
        const publicKey = new PublicKey(account);
        const balance = await connection.getBalance(publicKey);
        setBalance(balance / 1e9); // Convert lamports to SOL
      }
    };
    loadBalance();
  }, [account]);

  return (
    <div className="bg-white p-6 shadow-lg rounded-md mt-4">
      {account ? (
        <div>
          <h2 className="text-xl font-semibold">Account: {account}</h2>
          <p className="mt-2 text-lg">Balance: {balance} SOL</p>
        </div>
      ) : (
        <p>Please connect your wallet to view balance.</p>
      )}
    </div>
  );
}

export default WalletBalance;
