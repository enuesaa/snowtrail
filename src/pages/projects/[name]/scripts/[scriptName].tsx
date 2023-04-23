import { ScriptDetail } from '@/components/project/ScriptDetail'
import { useRouter } from 'next/router'

export default function Page() {
  const router = useRouter()
  const { name: projectName, scriptName } = router.query
  if (typeof projectName !== 'string' || typeof scriptName !== 'string') {
    return <></>
  }

  return (
    <>
      <ScriptDetail name={scriptName} />
    </>
  )
}
