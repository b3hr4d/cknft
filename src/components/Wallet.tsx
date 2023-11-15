import React from "react"
import { sepolia, useAccount, useConnect, useDisconnect } from "wagmi"
import { MetaMaskConnector } from "wagmi/connectors/metaMask"
import { WalletConnectConnector } from "wagmi/connectors/walletConnect"
import Shop from "./Shop"
import Receive from "./Receive"

// import { AuthClient } from "@dfinity/auth-client";
// import icblast, { internetIdentity } from "@infu/icblast";

// const identity = AuthClient.getIdentity();
// console.log(identity.getPrincipal().toText());

// let canister_id = "";

// let ic = icblast({ identity });
// let can = await ic(canister_id);


const projectId = process.env.NEXT_PUBLIC_WC_PROJECT_ID!

interface WalletProps {}

const Wallet: React.FC<WalletProps> = ({}) => {
  const [principalId, setPrincipalId] = React.useState("");
    const [accountId, setAccountId] = React.useState("");
    const [showRecievePortal, setShowRecievePortal] = React.useState(false);
    const [isValid, setIsValid] = React.useState(true); // State variable for input validity
    const [showTransactions, setShowTransactions] = React.useState(false); // State variable for the fetched data

    // return to main screen
    function goBack () {
      setShowRecievePortal(false);
    }

    // display transactions
    function displayTransactions () {
      setShowTransactions(!showTransactions);
    }

  const { address } = useAccount()

  const { connect: walletConnect } = useConnect({
    connector: new WalletConnectConnector({
      chains: [sepolia],
      options: {
        projectId,
        metadata: {
          name: "ICPPayment",
          description: "Internet Computer Payment",
          url: "https://github.com/B3Pay",
          icons: ["https://avatars.githubusercontent.com/u/121541974"]
        }
      }
    })
  })

  const { connect: metamask } = useConnect({
    connector: new MetaMaskConnector()
  })


  const { disconnect } = useDisconnect()

  if (address)
    return (
      <main>
        Connected to: {address}
        <button onClick={() => disconnect()}>Disconnect</button>
        <Shop />
        {/* <Receive principalId={principalId} accountId={accountId} showTransactions={showTransactions} displayTransactions={displayTransactions} goBack={goBack}> </Recieve> */}
      </main>
    )
  return (
    <div>
      <button onClick={() => metamask()}>Metamask</button> 
      <button onClick={() => walletConnect()}>WalletConnect</button>
    </div>
  )
}

export default Wallet
