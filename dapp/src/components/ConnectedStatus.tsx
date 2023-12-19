// ConnectedStatus.tsx
import React from 'react';

interface ConnectedStatusProps {
  connected: string;
}

const ConnectedStatus: React.FC<ConnectedStatusProps> = ({ connected }) => {
  return connected !== 'default' ? <h3>{connected}</h3> : null;
};

export default ConnectedStatus;
