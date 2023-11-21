import Head from "next/head"

import styles from "styles/Home.module.css"

import Login from "components/Login"
import Wallet from "components/Wallet"
import Image from "next/image"
import { config } from "service/config"
import { WagmiConfig } from "wagmi"

function HomePage() {
  return (
    <div className={styles.container}>
      <Head>
        <title>ckNFT</title>
      </Head>
      <main className={styles.main}>
        <h3 className={styles.title}>Direct EVM ckNFT for ICRC7</h3>
        <Login />
        <WagmiConfig config={config}>
          <Wallet />
        </WagmiConfig>
      </main>
      <footer className={styles.footer}>
        <a
          href="https://internetcomputer.org/"
          target="_blank"
          rel="noopener noreferrer"
        >
          <Image
            width={140}
            height={30}
            src="/icp-logo.svg"
            alt="DFINITY logo"
            className={styles.logo}
          />
        </a>
      </footer>
    </div>
  )
}

export default HomePage
