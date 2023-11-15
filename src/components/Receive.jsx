import React from 'react';
import QRCode from "react-qr-code";
import { fetchTransactionsCKBTC, fetchTransactionsICP, PrincipalToAccountIdText } from './utils';
import Transaction from './Transaction';
import SHA256 from 'crypto-js/sha256';
import Popup from './Popup';
import { Principal } from '@dfinity/principal';
//import { useAuth } from './use-auth-client';

const CKBTC_CANISTER_ID = "mxzaz-hqaaa-aaaar-qaada-cai";
const TRANSACTION_LIMIT = 10;
const POLLING_INTERVAL = 8000;

const logoStyles = {
  flex: "0 0 auto",
  width: "34px",
  height: "20px",
};

const trigger_notification = async (ctx) =>{
  console.log("trigger_notification")
  const thing = await ctx.ifttt_webhook().catch(ex => {
    alert(ex);
    return false;
  }).then(x => {
    console.log(x);
  });
}

const compare_icp_data = (old_data, new_data) => {
  if(!old_data || !new_data){
    return false;
  }
  var old_count = old_data.total_count;
  var new_count = new_data.total_count;
  // console.log("old icp count: " + old_count);
  // console.log("new icp count: " + new_count);
  return old_count != new_count;
}

const compare_ckbtc_data = (old_data, new_data) => {
  if(!old_data || !new_data){
    return false;
  }
  const old_records = old_data.data.map(obj =>{
    let h = obj.to_account + obj.from_account + obj.amount + obj.index;
    const hashedData = SHA256(h).toString();    
    return hashedData;
  });

  const new_records = new_data.data.map(obj =>{
    let h = obj.to_account + obj.from_account + obj.amount + obj.index;
    const hashedData = SHA256(h).toString();    
    return hashedData;
  });
  // console.log(old_records)
  // console.log(new_records)
  if(JSON.stringify(old_records) != JSON.stringify(new_records)){
    return true;
  }else{
    return false;
  }
}

/**
 *
 * @param tx -  transaction object
 * @param principalId - principal address of user
 * @returns - json of transaction data 
 * 
 */
const getTransactionDataICP = (tx, principalId) => {
  const acc = PrincipalToAccountIdText(Principal.fromText(principalId));
  //console.log(tx.transaction);
  const tx_acc = tx.transaction?.operations[1]?.account?.address
  const val = tx.transaction?.operations[1]?.amount?.value
  const tx_type = acc === tx_acc  ? 'recieved' : 'sent';
  const tx_amt = val / 100000000;
  const tx_symbol = 'ICP'  
  return {
    type: tx_type,
    val: tx_amt,
    symbol: tx_symbol
  };
}

/**
 *
 * @param tx -  transaction object
 * @param principalId - principal address of user
 * @returns - json of transaction data 
 * 
 */
const getTransactionData = (tx, principalId) => {
  const tx_type = principalId === tx.to_account ? 'recieved' : 'sent';
  const tx_amt = tx.amount / 100000000;
  const tx_symbol = tx.ledger_canister_id === CKBTC_CANISTER_ID ? 'ckBTC' : 'ICP';  
  return {
    type: tx_type,
    val: tx_amt,
    symbol: tx_symbol
  };
}

function Recieve({ principalId, accountId, showTransactions, displayTransactions }) {

  const [dataICP, setDataICP] = React.useState(null);
  const [dataCKBTC, setDataCKBTC] = React.useState(null);
  const [showPopup, setShowPopup] = React.useState(false);
  const [popupMessage, setPopupMessage] = React.useState(null);
  const [showCKBTC, setShowCKBTC] = React.useState(true);
  const [listening, setListening] = React.useState(false);
  //const { actor } = useAuth();  

  // fetch data on initial mount
  React.useEffect(() => {
    const fetch = async () => {
      try {
        const ckbtcData = await fetchTransactionsCKBTC(principalId, TRANSACTION_LIMIT);
        const icpData = await fetchTransactionsICP(accountId, TRANSACTION_LIMIT);
        setDataCKBTC(ckbtcData);
        //console.log(ckbtcData);
        setDataICP(icpData);
        //console.log(icpData)
      } catch (error) {
        console.error(error);
      }
    };
    fetch();
  }, []);
  
  // poll api endpoint every POLLING_INTERVAL to see if new transactions came in 
  // if so, display a modal and push a notification 
  React.useEffect(() => {
    const timer = setInterval(async () => {
      setListening(true);
      console.log('refetch ckBTC and ICP');   
      const newCKBTCData = await fetchTransactionsCKBTC(principalId, TRANSACTION_LIMIT);
      //setDataCKBTC(newCKBTCData);
      const newICPData = await fetchTransactionsICP(accountId, TRANSACTION_LIMIT);
      //setDataICP(newICPData);
      var has_changed_ckBTC = compare_ckbtc_data(dataCKBTC, newCKBTCData)
      if(has_changed_ckBTC) {
        setDataCKBTC(newCKBTCData);
        const tx_data = getTransactionData(newCKBTCData.data[0]);
        setPopupMessage(tx_data);
        setShowPopup(true);
        //trigger_notification(actor); //server side https-outcall        
        setTimeout(() => {
          setShowPopup(false)
        }, 6000);
      };

      var has_changed_ICP = compare_icp_data(dataICP, newICPData)
      if(has_changed_ICP){
        setDataICP(newICPData);
        const tx_data = getTransactionDataICP(newICPData?.transactions[0], principalId)        
        setPopupMessage(tx_data);
        setShowPopup(true);
        //trigger_notification(actor); //server side https-outcall        
        setTimeout(() => {
          setShowPopup(false)
        }, 6000);
      };

    }, POLLING_INTERVAL);  
    return () => clearInterval(timer);
  }, [dataCKBTC, dataICP]);

  return (
    <div className="container">
      {showPopup && 
      <Popup 
        header_text="New Transaction!"
        body_text={`You ${popupMessage.type} a new transaction of ${popupMessage.val} ${popupMessage.symbol}`}
      />}
      <div className="smallContainer">
        
        <div style={{marginTop: "10px"}}>
          <QRCode value={principalId} />
        </div>

        <div className='rowContainer'>
          <h3 title={principalId}>{principalId.slice(0, 10) + "..."}</h3>
        </div>
        {dataCKBTC && showTransactions && showCKBTC &&
        <div className='borderless_container' >
          <h1>Recent Transactions: </h1>
          <>
            {dataCKBTC.data.length == 0 ? 
              <h3>No transactions yet!</h3> 
            : 
              (dataCKBTC?.data || []).map((transaction, index) => (
                <Transaction 
                  key={index} 
                  transaction={transaction} 
                  principalId={principalId}
                />
              ), [])
            }
          </>
        </div>}
      </div>
      <button type="button" id="addressButton" onClick={displayTransactions}>
        {showTransactions ? "Hide" : "Show"} Transactions
      </button>
      <div className="small_text">            
            {listening ? "listening for transactions..." : ""} 
        </div>
    </div>
  );
}

export default Recieve;
