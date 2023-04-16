import { Header } from '@/components/common/Header'
import { Main } from '@/components/common/Main'
import { Configure } from '@/components/project/Configure'
import { ConfigureAdd } from '@/components/project/ConfigureAdd'
import { useRouter } from 'next/router'

export default function Page() {
  const router = useRouter()
  const { create } = router.query

  return (
    <>
      <Header />
      <Main>
        {create === undefined ? <Configure /> : <ConfigureAdd />}
      </Main>
    </>
  )
}
