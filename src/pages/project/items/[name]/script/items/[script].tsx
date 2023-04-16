import { Header } from '@/components/common/Header'
import { useRouter } from 'next/router'
import { ScriptDetail } from '@/components/script/ScriptDetail'
import { Main } from '@/components/common/Main'

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
        <ScriptDetail />
      </Main>
    </>
  )
}
