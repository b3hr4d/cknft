import { useEffect } from "react"
import helperAbi from "service/abi.json"
import { useActorMethod } from "service/icrc7"
import { formatEther } from "viem"
import { useContractWrite } from "wagmi"
import Confirmation from "./Confirmation"

import styles from "styles/Item.module.css"

interface ItemProps {
  name: string
  price: bigint
}

const Item: React.FC<ItemProps> = ({ name, price }) => {
  const { data: canisterEthereumAddress, call } =
    useActorMethod("ethereum_address")

  useEffect(() => {
    call()
  }, [])

  const { data, isLoading, write } = useContractWrite({
    address: "0xb44B5e756A894775FC32EDdf3314Bb1B1944dC34",
    abi: helperAbi,
    functionName: "deposit",
    value: price,
    args: [canisterEthereumAddress]
  })

  return (
    <div className={styles.item}>
      {isLoading ? (
        <div>Buying {name}…</div>
      ) : data?.hash ? (
        <Confirmation hash={data.hash} item={name} />
      ) : (
        <>
          <h3>{name}</h3>
          <div>{formatEther(price).toString()} USDE</div>
          <button onClick={() => write()}>Buy {name}</button>
        </>
      )}
    </div>
  )
}

export default Item
