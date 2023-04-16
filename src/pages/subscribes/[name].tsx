import { Header } from '@/components/common/Header'
import { useRouter } from 'next/router'
import { Detail } from '@/components/subscribe/Detail'
import { Main } from '@/components/common/Main'

export default function Page() {
  const router = useRouter()
  const { name } = router.query
  if (typeof name !== 'string') {
    return <></>
  }

  return (
    <>
      <Header />
      <Main>
        <Detail />
      </Main>
    </>
  )
}
