// FungibleTokens.tsx
import React from 'react';

interface FungibleToken {
  type: string;
  address: string;
  value: string;
}

interface FungibleTokensProps {
  tokens: { [key: string]: FungibleToken };
  onSelectToken: (address: string) => void;
  onValueChange: (address: string, newValue: number) => void;
  selectedAddresses: string[];
  addressValues: { [key: string]: number };
}

const FungibleTokens: React.FC<FungibleTokensProps> = ({ tokens, onSelectToken, onValueChange, selectedAddresses, addressValues }) => {
  
  function printAbbreviatedString(inputString: string) {
    if (inputString.length <= 4) {
      // If the string is already short, just print it as is
      return inputString

    } else {
      // Print the abbreviated version with dots in the middle
      const abbreviatedString = `${inputString.slice(0, 4)}...${inputString.slice(-6)}`;
      return abbreviatedString
    }
  }
  
  return (
    <div>
      <h2>Fungible Tokens: </h2>
      {Object.entries(tokens).map(([address, token]) => (
        <div key={address}>
          <label>
            <input
              type="checkbox"
              checked={selectedAddresses.includes(address)}
              onChange={() => onSelectToken(address)}
            />
            {` ${printAbbreviatedString(address)} - Value: ${token.value}`}
          </label>
          {selectedAddresses.includes(address) && (
            <div>
              <label>
                <input
                  type="number"
                  value={addressValues[address] || ''}
                  onChange={(e) => onValueChange(address, parseInt(e.target.value, 10))}
                />
              </label>
            </div>
          )}
        </div>
      ))}
    </div>
  );
};

export default FungibleTokens;
