import { ScriptDetail } from '@/components/project/ScriptDetail'
import { useRouter } from 'next/router'

export default function Page() {
  const router = useRouter()
  const { projectId, scriptId } = router.query
  if (typeof projectId !== 'string' || typeof scriptId !== 'string') {
    return <></>
  }

  return (
    <>
      <ScriptDetail id={scriptId} />
    </>
  )
}
