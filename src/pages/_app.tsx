import { Layout } from '@/components/common/Layout'
import { globalStyle } from '@/styles/global'
import { baseTheme } from '@/styles/theme'
import { Global, ThemeProvider } from '@emotion/react'
import type { NextPage } from 'next'
import type { AppProps } from 'next/app'
import type { ReactElement, ReactNode } from 'react'

export type NextPageWithLayout<P = {}, IP = P> = NextPage<P, IP> & {
  getLayout?: (page: ReactElement) => ReactNode
}

type Props = AppProps & {
  Component: NextPageWithLayout
}
export default function App({ Component, pageProps }: Props) {
  const getLayout = Component.getLayout ?? ((page) => <Layout>{page}</Layout>)

  return (
    <>
      <Global styles={globalStyle} />
      <ThemeProvider theme={baseTheme}>{getLayout(<Component {...pageProps} />)}</ThemeProvider>
    </>
  )
}
