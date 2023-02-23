import type { AppProps } from 'next/app'
import { Global } from '@emotion/react'
import { globalStyle } from '../styles/global'
import { ThemeProvider } from '@emotion/react'
import { baseTheme } from '../styles/theme'

export default function App({ Component, pageProps }: AppProps) {
  return (
    <>
      <Global styles={globalStyle} />
      <ThemeProvider theme={baseTheme}>
        <Component {...pageProps} />
      </ThemeProvider>
    </>
  )
}
