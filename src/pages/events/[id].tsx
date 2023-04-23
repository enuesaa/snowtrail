import { useRouter } from 'next/router'
import { Detail } from '@/components/event/Detail'

export default function Page() {
  const router = useRouter()
  const { id } = router.query
  if (typeof id !== 'string') {
    return <></>
  }

  return (
    <>
      <Detail id={id} />    
    </>
  )
}
