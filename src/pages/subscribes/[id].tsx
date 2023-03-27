import { Header } from '@/components/common/Header'
import { useRouter } from 'next/router'
import { SubscribeDetail } from '@/components/subscribe/SubscribeDetail'

export default function () {
  const router = useRouter()
  const { id } = router.query
  if (typeof id !== 'string') {
    return (<></>)
  }

  return (
    <>
      <Header />
      <SubscribeDetail />
    </>
  )
}
