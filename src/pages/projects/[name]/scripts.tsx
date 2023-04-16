import { Header } from '@/components/common/Header'
import { Main } from '@/components/common/Main'
import { ScriptList } from '@/components/project/ScriptList'
import { ScriptAdd } from '@/components/project/ScriptAdd'
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
        {create === undefined ?  <ScriptList projectName={projectName} /> : <ScriptAdd projectName={projectName} />}
      </Main>
    </>
  )
}
