"use client";
import React, { useState } from "react";
import { useBalance } from "@starknet-react/core";
import { useRouter } from "next/navigation";
import { useAppContext } from "@/context/walletContext";
import { WalletConnectModal } from "../WalletConnectModal";
const Connector: React.FC = () => {
  const { address, status } = useAppContext();
  const [isModalOpen, setIsModalOpen] = useState(false);
  const { data } = useBalance({
    token: "0x04718f5a0Fc34cC1AF16A1cdee98fFB20C31f5cD61D6Ab07201858f4287c938D",
    address: address as "0x",
  });
  const router = useRouter();

  console.log(data);
  console.log(status);
  return (
    <div className="p-6 text-center">
      {status === "connecting" && (
        <p className="text-lg font-medium text-gray-500">Connecting...</p>
      )}
      {status === "disconnected" && (
        <div>
          <button
            onClick={() => setIsModalOpen(true)}
            className="px-4 py-2 rounded-full bg-gradient-to-r from-teal-500 to-blue-700 hover:from-teal-600 hover:to-blue-800 text-white font-medium"
          >
            Connect Wallet
          </button>
          <WalletConnectModal 
            isOpen={isModalOpen} 
            onClose={() => setIsModalOpen(false)} 
          />
        </div>
      )}
      {status === "connected" && address && (
        <div className="text-white">
          <p>Connected: {address.slice(0, 6)}...{address.slice(-4)}</p>
          {data && <p>Balance: {data.formatted} STRK</p>}
        </div>
      )}
    </div>
  );
};

export default Connector;
