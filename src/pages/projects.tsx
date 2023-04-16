import { Add } from '@/components/project/Add'
import { List } from '@/components/project/List'
import { useRouter } from 'next/router'

export default function Page() {
  const router = useRouter()
  const { create } = router.query

  return <>{create === undefined ? <List /> : <Add />}</>
}
