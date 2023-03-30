import { Header } from '@/components/common/Header'
import { Databoard } from '@/components/setting/Databoard'
import { Main } from '@/components/common/Main'

export default function Page() {
  return (
    <>
      <Header />
      <Main>
        <Databoard />
      </Main>
    </>
  )
}
