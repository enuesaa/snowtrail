import { Detail } from '@/components/project/Detail'
import { useRouter } from 'next/router'

export default function Page() {
  const router = useRouter()
  const { projectId } = router.query
  if (typeof projectId !== 'string') {
    return <></>
  }

  return (
    <>
      <Detail id={projectId} />
    </>
  )
}
