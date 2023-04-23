import { Add } from '@/components/project/Add'
import { useRouter } from 'next/router'

export default function Page() {
  const router = useRouter()
  const { create } = router.query

  return <>{create === undefined ? <></> : <Add />}</>
}
