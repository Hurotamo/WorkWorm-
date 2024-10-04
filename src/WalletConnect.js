import { useState } from 'react';

function WalletConnect() {
  const [account, setAccount] = useState(null);

  const connectWallet = async () => {
    if (window.solana) {
      try {
        const accounts = await window.solana.request({ method: 'connect' });
        setAccount(accounts.publicKey.toString());
      } catch (error) {
        console.error("User rejected the request:", error);
      }
    } else {
      alert('Phantom wallet not found');
    }
  };

  return (
    <div className="bg-white p-6 shadow-lg rounded-md">
      <button
        onClick={connectWallet}
        className="px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 focus:outline-none"
      >
        Connect Wallet
      </button>
      {account && (
        <p className="mt-4 text-gray-800">Connected account: {account}</p>
      )}
    </div>
  );
}

export default WalletConnect;
