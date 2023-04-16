import { Header } from '@/components/common/Header'
import { Main } from '@/components/common/Main'
import { Configure } from '@/components/script/Configure'
import { ConfigureAdd } from '@/components/script/ConfigureAdd'
import { useRouter } from 'next/router'

export default function Page() {
  const router = useRouter()
  const { name: projectName, create } = router.query
  if (typeof projectName !== 'string') {
    return <></>
  }

  return (
    <>
      <Header />
      <Main>
        {create === undefined ?  <Configure /> : <ConfigureAdd projectName={projectName} />}
        <Configure />
      </Main>
    </>
  )
}
