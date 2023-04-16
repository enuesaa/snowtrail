import { Header } from '@/components/common/Header'
import { useRouter } from 'next/router'
import { ScriptDetail } from '@/components/project/ScriptDetail'
import { Main } from '@/components/common/Main'

export default function Page() {
  const router = useRouter()
  const { name: projectName, scriptName } = router.query
  if (typeof projectName !== 'string' || typeof scriptName !== 'string') {
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
