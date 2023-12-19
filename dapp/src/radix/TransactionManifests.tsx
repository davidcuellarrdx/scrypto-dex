import React, { useEffect, useState } from "react";

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

interface TransactionManifestsProps {
  selectedAccount: number;
  sendTransaction: (transaction: string) => void;
  accounts: AccountType; // Replace YourAccountType with the actual type of your accounts
  selectedAddresses: string[];
  addressValues: { [key: string]: number };
  selectedButton: string;
}

const TransactionManifests: React.FC<TransactionManifestsProps> = ({
  selectedAccount,
  sendTransaction,
  accounts,
  selectedAddresses,
  addressValues,
  selectedButton,
}) => {
  const [swapAddress, setSawpAddress] = useState<string | null>(null);

  const transactionAddLiquidity = `
      # Withdraw token1 from primary account
        CALL_METHOD
            Address("${
              accounts.state.accounts[selectedAccount].address
            }") #Primary account
            "withdraw"
            Address("${selectedAddresses[0]}") # token1 address
            Decimal("${addressValues[selectedAddresses[0]]}");
      
        # Withdraw token2 from primary account
        CALL_METHOD
            Address("${
              accounts.state.accounts[selectedAccount].address
            }") #Primary account
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

  const transactionSwap = `
      # Withdraw token1 from primary account
        CALL_METHOD
            Address("${
              accounts.state.accounts[selectedAccount].address
            }") #Primary account
            "withdraw"
            Address("${selectedAddresses[0]}") # token1 address
            Decimal("${addressValues[selectedAddresses[0]]}");
      
        # Put the token1 from worktop into bucket_a
        TAKE_FROM_WORKTOP
            Address("${selectedAddresses[0]}") # token1 address
            Decimal("${addressValues[selectedAddresses[0]]}")
            Bucket("bucket_a");
            
        # Call swap method
        CALL_METHOD 
            Address("component_tdx_2_1czu5t695smlmpj4emghmf53392ff8p32ewk963e8qw8uvascq2j9w2") # Component_address
            "swap" # "method_name"
            Bucket("bucket_a")
            Address("${swapAddress}");
      
      # Because we withdrew tokens from our account and they could still be on the
      # worktop, we have to deposit them back into your account
          CALL_METHOD
          Address("${accounts.state.accounts[selectedAccount].address}") 
          "try_deposit_batch_or_abort"
          Expression("ENTIRE_WORKTOP")
          None;
      `;

  const transactionRemoveLiquidity = `
      # Withdraw token1 from primary account
        CALL_METHOD
            Address("${
              accounts.state.accounts[selectedAccount].address
            }") #Primary account
            "withdraw"
            Address("${selectedAddresses[0]}") 
            Decimal("${addressValues[selectedAddresses[0]]}");
      
        # Put the token1 from worktop into bucket_a
        TAKE_FROM_WORKTOP
            Address("${selectedAddresses[0]}") 
            Decimal("${addressValues[selectedAddresses[0]]}")
            Bucket("bucket_a");

        # Call remove liquidity method
        CALL_METHOD 
            Address("component_tdx_2_1czu5t695smlmpj4emghmf53392ff8p32ewk963e8qw8uvascq2j9w2") # Component_address
            "remove_liquidity" # "method_name"
            Bucket("bucket_a");
      
      # Because we withdrew tokens from our account and they could still be on the
      # worktop, we have to deposit them back into your account
          CALL_METHOD
          Address("${accounts.state.accounts[selectedAccount].address}") 
          "try_deposit_batch_or_abort"
          Expression("ENTIRE_WORKTOP")
          None;
      `;

  const handleCreateFungibleResource = (selectedButton: string) => {
    if (selectedButton === "Add Liquidity") {
      sendTransaction(transactionAddLiquidity);
      //console.log(transactionAddLiquidity)
    } else if (selectedButton === "Swap") {
      sendTransaction(transactionSwap);
    } else if (selectedButton === "Remove Liquidity") {
      sendTransaction(transactionRemoveLiquidity);
    } else {
      return null;
    }
  };

  const handleDisable = (selectedButton: string) => {
    if (
      selectedButton === "Add Liquidity" &&
      (addressValues[selectedAddresses[0]] === undefined ||
        addressValues[selectedAddresses[1]] === undefined)
    ) {
      return true;
    } else if (
      (selectedButton === "Swap" || selectedButton === "Remove Liquidity") &&
      addressValues[selectedAddresses[0]] === undefined
    ) {
      return true;
    } else {
      false;
    }
  };

  return (
    <div>
      {selectedButton === "Swap" && (
        <div>
          <h4>Swap to ResourceAddress </h4>
          <label>
            <input
              type="text"
              value={swapAddress || ""}
              onChange={(e) => setSawpAddress(e.target.value)}
            />
          </label>
        </div>
      )}

      <button
        style={{ display: "block", margin: 10, width: "100%" }}
        onClick={() => handleCreateFungibleResource(selectedButton)}
        disabled={handleDisable(selectedButton)}
      >
        Send transaction
      </button>
    </div>
  );
};

export default TransactionManifests;
