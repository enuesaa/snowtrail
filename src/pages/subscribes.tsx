import { Header } from '@/components/common/Header'
import { Main } from '@/components/common/Main'
import { Add } from '@/components/subscribe/Add'
import { List } from '@/components/subscribe/List'
import { useRouter } from 'next/router'

export default function Page() {
  const router = useRouter()
  const { create } = router.query

  return <>{create === undefined ? <List /> : <Add />}</>
}
