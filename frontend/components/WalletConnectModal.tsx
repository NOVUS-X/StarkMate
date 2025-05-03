import { Button } from "@/components/ui/button";
import { useAppContext } from "@/context/walletContext";
import { useConnect } from "@starknet-react/core";
import { useState } from "react";

interface WalletConnectModalProps {
  isOpen: boolean;
  onClose: () => void;
}

export function WalletConnectModal({
  isOpen,
  onClose,
}: WalletConnectModalProps) {
  const { connectors } = useConnect();
  const { connectWallet } = useAppContext();
  if (!isOpen) return null;

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center">
      <div
        className="fixed inset-0 bg-black/50 backdrop-blur-sm"
        onClick={onClose}
      />

      <div className="relative bg-gray-900 rounded-lg border border-gray-800 p-6 w-full max-w-md mx-4">
        <div className="space-y-4">
          <div className="text-center">
            <h2 className="text-xl font-bold text-white">Connect Wallet</h2>
            <p className="text-gray-400 mt-2">
              Choose your preferred wallet to connect to StarkMate
            </p>
          </div>
          <div className="space-y-3">
            {connectors.map((connector, index) => {
              return (
                <Button
                  key={`${connector.id}${index}$`}
                  className="w-full bg-gradient-to-r from-teal-500 to-blue-700 hover:from-teal-600 hover:to-blue-800"
                  onClick={() => {
                    connectWallet(connector)
                     onClose()
                  }
                  
                  }
                >
                  {connector.id.charAt(0).toUpperCase() + connector.id.slice(1)}
                </Button>
              );
            })}
          </div>
        </div>
      </div>
    </div>
  );
}
