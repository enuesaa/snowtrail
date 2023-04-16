import { ScriptAdd } from '@/components/project/ScriptAdd'
import { ScriptList } from '@/components/project/ScriptList'
import { useRouter } from 'next/router'

export default function Page() {
  const router = useRouter()
  const { name: projectName, create } = router.query
  if (typeof projectName !== 'string') {
    return <></>
  }

  return (
    <>{create === undefined ? <ScriptList projectName={projectName} /> : <ScriptAdd projectName={projectName} />}</>
  )
}
