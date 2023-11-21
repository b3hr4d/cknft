import { useEffect } from "react"
import { useActorMethod } from "service/icrc7"
import Item from "./Item"

import styles from "styles/Shop.module.css"

interface ShopProps {}

const Shop: React.FC<ShopProps> = ({}) => {
  const {
    data: items,
    loading,
    call
  } = useActorMethod("icrc7_collection_metadata")

  useEffect(() => {
    call()
  }, [])

  return (
    <div className={styles.container}>
      {loading ? (
        <div>Loading...</div>
      ) : (
        items &&
        Object.entries(items)?.map(([name, price]) => {
          return <Item name={name} price={price} key={name} />
        })
      )}
    </div>
  )
}

export default Shop
