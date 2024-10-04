import React from 'react';
import WalletConnect from './WalletConnect';
import WalletBalance from './WalletBalance';

function App() {
  return (
    <div className="flex flex-col items-center justify-center min-h-screen bg-gray-100">
      <h1 className="text-4xl font-bold text-blue-600 mb-4">WorkWorm</h1>
      <h2 className="text-xl font-semibold mb-8">A DECENTRALIZED Freelance Marketplace</h2>
      <WalletConnect />
      <WalletBalance />
    </div>
  );
}

export default App;
