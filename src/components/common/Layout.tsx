import { Header } from '@/components/common/Header'
import { Main } from '@/components/common/Main'
import { ReactNode } from 'react'

type Props = {
  children: ReactNode
}
export const Layout = ({ children }: Props) => {
  return (
    <>
      <Header />
      <Main>{children}</Main>
    </>
  )
}
