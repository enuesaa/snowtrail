import { Header } from '@/components/common/Header'
import { Main } from '@/components/common/Main'
import { Configure } from '@/components/script/Configure'

export default function Page() {
  return (
    <>
      <Header />
      <Main>
        <Configure />
      </Main>
    </>
  )
}
