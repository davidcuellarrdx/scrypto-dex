import { RadixDappToolkit, RadixNetwork } from "@radixdlt/radix-dapp-toolkit";
import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import "./index.css";
import { RdtProvider } from "./RdtProvider";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <RdtProvider
      value={RadixDappToolkit({
        dAppDefinitionAddress:
          'account_tdx_2_12863lweeadwudz0lz4dhvzfd9za6l99ncsrj94758ttahxukerct3v', //Use your dAppDefinitionAddress
        networkId: RadixNetwork.Stokenet, //RadixNetwork.Mainnet
        applicationName: 'ScryptoDex dApp',
        applicationVersion: '1.0.0',
      })}
    >
      <App />
    </RdtProvider>
  </React.StrictMode>
);
