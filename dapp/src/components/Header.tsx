// Header.tsx
import React from 'react';

const Header: React.FC = () => {
  return (
    <div>
      <h1>Scrypto DEX</h1>
      <div className="card">
        <radix-connect-button />
      </div>
    </div>
  );
};

export default Header;
