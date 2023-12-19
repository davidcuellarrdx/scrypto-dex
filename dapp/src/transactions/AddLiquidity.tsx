// AddLiquidity.tsx
import React, { useEffect, useState } from 'react';

type AccountType = {
    state: {
      accounts: {
        address: string;
        label: string;
        appearanceId: number;
        fungibleTokens: { [key: string]: FungibleToken }; // Define FungibleToken type
        nonFungibleTokens: { [key: string]: NonFungibleToken[] }; // Define NonFungibleToken type
      }[];
      status: string;
      hasLoaded: boolean;
    };
  };

  type FungibleToken = {
    type: string;
    address: string;
    value: string;
    // ... other properties
  };
  
  type NonFungibleToken = {
    type: string;
    id: string;
    address: string;
    // ... other properties
  };

interface AddLiquidityProps {
  selectedAccount: number | null;
  sendTransaction: (transaction: string) => void;
  accounts: AccountType; // Replace YourAccountType with the actual type of your accounts
  selectedAddresses: string[];
  addressValues: { [key: string]: number };
}

const AddLiquidity: React.FC<AddLiquidityProps> = ({ selectedAccount, sendTransaction, accounts, selectedAddresses, addressValues  }) => {
    
  
  
    const handleCreateFungibleResource = () => {
    if (selectedAccount !== null && addressValues[selectedAddresses[0]] !== undefined && addressValues[selectedAddresses[1]] !== undefined) {
      const transactionScript = `
      # Withdraw token1 from primary account
        CALL_METHOD
            Address("${accounts.state.accounts[selectedAccount].address}") #Primary account
            "withdraw"
            Address("${selectedAddresses[0]}") # token1 address
            Decimal("${addressValues[selectedAddresses[0]]}");
      
        # Withdraw token2 from primary account
        CALL_METHOD
            Address("${accounts.state.accounts[selectedAccount].address}") #Primary account
            "withdraw"
            Address("${selectedAddresses[1]}") # token2 address
            Decimal("${addressValues[selectedAddresses[1]]}");
      
        # Put the token1 from worktop into bucket_a
        TAKE_FROM_WORKTOP
            Address("${selectedAddresses[0]}") # token1 address
            Decimal("${addressValues[selectedAddresses[0]]}")
            Bucket("bucket_a");
            
      # Put the token2 from worktop into bucket_b
      TAKE_FROM_WORKTOP
          Address("${selectedAddresses[1]}") # token2 address
          Decimal("${addressValues[selectedAddresses[1]]}")
          Bucket("bucket_b");
      
      # Call add liquidity method
      CALL_METHOD 
          Address("component_tdx_2_1czu5t695smlmpj4emghmf53392ff8p32ewk963e8qw8uvascq2j9w2") # Component_address
          "add_liquidity" # "method_name"
          Bucket("bucket_a")
          Bucket("bucket_b");
      
      # Because we withdrew tokens from our account and they could still be on the
      # worktop, we have to deposit them back into your account
          CALL_METHOD
          Address("${accounts.state.accounts[selectedAccount].address}") 
          "try_deposit_batch_or_abort"
          Expression("ENTIRE_WORKTOP")
          None;
      `;

      console.log(transactionScript);

      sendTransaction(transactionScript);
    }
  };

  return (
    <div>
      <button
        style={{ display: 'block', margin: 10, width: '100%' }}
        onClick={handleCreateFungibleResource}
        disabled={selectedAccount === null || addressValues[selectedAddresses[0]] === undefined || addressValues[selectedAddresses[1]] === undefined}
      >
        Send transaction
      </button>
    </div>
  );
};

export default AddLiquidity;
