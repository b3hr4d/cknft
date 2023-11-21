import { AppProps } from "next/app"
import React from "react"
import "styles/global.css"
import { ReActorProvider } from "../service/icrc7"

const App: React.FC<AppProps> = ({ Component, pageProps }) => {
  return (
    <ReActorProvider>
      <br />
      <Component {...pageProps} />
    </ReActorProvider>
  )
}

export default App
