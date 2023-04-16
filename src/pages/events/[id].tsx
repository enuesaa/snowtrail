import { useRouter } from 'next/router'

export default function Page() {
  const router = useRouter()
  const { id } = router.query
  if (typeof id !== 'string') {
    return <></>
  }

  return <>{id}</>
}
