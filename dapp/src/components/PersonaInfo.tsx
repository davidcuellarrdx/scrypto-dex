// PersonaInfo.tsx
import React from "react";

interface Account {
  // Define the Account type based on your application's structure
  label: string;
  // ... other properties
}

interface PersonaInfoProps {
  label: string;
  accounts: Account[];
  setSelectAccount: (index: number) => void;
  selectedAccount: number | null;
}

const PersonaInfo: React.FC<PersonaInfoProps> = ({
  label,
  accounts,
  setSelectAccount,
  selectedAccount,
}) => {
  return (
    <div>
      <h2>Persona: {label}</h2>
      {accounts.length > 0 ? (
        <div style={{ marginBottom: 25 }}>
          <h2>Select account: </h2>
          {accounts.map((account, index) => (
            <div
              key={index}
              style={{ cursor: "pointer", marginBottom: 10 }}
              onClick={() => setSelectAccount(index)}
            >
              <input
                type="radio"
                readOnly
                checked={selectedAccount === index}
              />
              {account.label}
            </div>
          ))}
        </div>
      ) : (
        <h4>
          Click on the Wallet button and update data sharing and choose at least
          1 account.
        </h4>
      )}
    </div>
  );
};

export default PersonaInfo;
