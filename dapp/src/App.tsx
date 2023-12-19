import React, { useState } from "react";
import "./App.css";
import Header from "./components/Header";
import ConnectedStatus from "./components/ConnectedStatus";
import PersonaInfo from "./components/PersonaInfo";
import FungibleTokens from "./components/FungibleTokens";
import { useAccounts } from "./hooks/useAccounts";
import { usePersona } from "./hooks/usePersona";
import { useConnected } from "./hooks/useConnected";
import { useSendTransaction } from "./hooks/useSendTransaction";
import TransactionManifests from "./radix/TransactionManifests";

declare global {
  namespace JSX {
    interface IntrinsicElements {
      "radix-connect-button": React.DetailedHTMLProps<
        React.HTMLAttributes<HTMLElement>,
        HTMLElement
      >;
    }
  }
}

function App() {
  const accounts = useAccounts();
  const persona = usePersona();
  const sendTransaction = useSendTransaction();
  const connected = useConnected();

  const [selectedAccount, setSelectedAccount] = useState<number | null>(null);

  const [selectedAddresses, setSelectedAddresses] = useState<string[]>([]);
  const [addressValues, setAddressValues] = useState<{ [key: string]: number }>(
    {}
  );

  const [selectedButton, setSelectedButton] = useState<string | null>(null);

  const handleCheckboxChange = (address: string) => {
    if (selectedAddresses.includes(address)) {
      setSelectedAddresses(
        selectedAddresses.filter(
          (selectedAddress) => selectedAddress !== address
        )
      );
    } else {
      if (selectedButton === "Add Liquidity" && selectedAddresses.length < 2) {
        setSelectedAddresses([...selectedAddresses, address]);
      }
      if (
        (selectedButton === "Swap" || selectedButton === "Remove Liquidity") &&
        selectedAddresses.length < 1
      ) {
        setSelectedAddresses([...selectedAddresses, address]);
      }
    }
  };

  const handleValueChange = (address: string, newValue: number) => {
    setAddressValues({ ...addressValues, [address]: newValue });
  };

  const handleButtonClick = (button: string) => {
    setSelectedButton(button);
    setSelectedAddresses([]);
    setAddressValues({});
  };

  //console.log(selectedAccount)

  return (
    <div className="App">
      <Header />
      <ConnectedStatus connected={connected} />

      {persona.persona ? (
        <PersonaInfo
          label={persona.persona.label}
          accounts={accounts.state.accounts}
          setSelectAccount={setSelectedAccount}
          selectedAccount={selectedAccount}
        />
      ) : null}

      {selectedAccount !== null ? (
        <div>
          <div style={{ margin: 10 }}>
            <button
              onClick={() => handleButtonClick("Add Liquidity")}
              style={{
                backgroundColor:
                  selectedButton === "Add Liquidity" ? "green" : "#646cfff9",
                margin: 10,
              }}
            >
              Add Liquidity
            </button>
            <button
              onClick={() => handleButtonClick("Swap")}
              style={{
                backgroundColor:
                  selectedButton === "Swap" ? "green" : "#646cfff9",
                margin: 10,
              }}
            >
              Swap
            </button>
            <button
              onClick={() => handleButtonClick("Remove Liquidity")}
              style={{
                backgroundColor:
                  selectedButton === "Remove Liquidity" ? "green" : "#646cfff9",
                margin: 10,
              }}
            >
              Remove Liquidity
            </button>
          </div>

          {selectedButton === "Add Liquidity" && (
            <h3> Select two resources </h3>
          )}

          {(selectedButton === "Swap" ||
            selectedButton === "Remove Liquidity") && (
            <h3> Select one resource </h3>
          )}

          {selectedButton && (
            <div>
              <FungibleTokens
                tokens={accounts.state.accounts[selectedAccount].fungibleTokens}
                onSelectToken={handleCheckboxChange}
                onValueChange={handleValueChange}
                selectedAddresses={selectedAddresses}
                addressValues={addressValues}
              />
              <TransactionManifests
                selectedAccount={selectedAccount}
                sendTransaction={sendTransaction}
                accounts={accounts}
                selectedAddresses={selectedAddresses}
                addressValues={addressValues}
                selectedButton={selectedButton}
              />
            </div>
          )}
        </div>
      ) : (
        <h4>Don't forget to enable Developer Mode on your Radix Wallet</h4>
      )}
    </div>
  );
}

export default App;
