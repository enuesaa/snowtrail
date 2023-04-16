import { Detail } from '@/components/subscribe/Detail'
import { useRouter } from 'next/router'

export default function Page() {
  const router = useRouter()
  const { name } = router.query
  if (typeof name !== 'string') {
    return <></>
  }

  return (
    <>
      <Detail />
    </>
  )
}