import { Header } from '@/components/common/Header'
import { Main } from '@/components/common/Main'
import { List } from '@/components/subscribe/List'
import { useRouter } from 'next/router'
import { Add } from '@/components/subscribe/Add'

export default function Page() {
  const router = useRouter()
  const { create } = router.query

  return (
    <>
      <Header />
      <Main>
        {create === undefined ? <List /> : <Add />}
      </Main>
    </>
  )
}
