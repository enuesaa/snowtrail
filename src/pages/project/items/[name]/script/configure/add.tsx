import { Header } from '@/components/common/Header'
import { Main } from '@/components/common/Main'
import { ConfigureAdd } from '@/components/script/ConfigureAdd'
import { useRouter } from 'next/router'

export default function Page() {
  const router = useRouter()
  const { name: projectName } = router.query
  if (typeof projectName !== 'string') {
    return <></>
  }

  return (
    <>
      <Header />
      <Main>
        <ConfigureAdd projectName={projectName} />
      </Main>
    </>
  )
}
