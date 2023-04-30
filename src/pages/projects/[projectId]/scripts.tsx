import { ScriptAdd } from '@/components/project/ScriptAdd'
import { useRouter } from 'next/router'

export default function Page() {
  const router = useRouter()
  const { projectId, create } = router.query
  if (typeof projectId !== 'string') {
    return <></>
  }

  return (
    <>{create === undefined ? <></> : <ScriptAdd projectId={projectId} />}</>
  )
}
