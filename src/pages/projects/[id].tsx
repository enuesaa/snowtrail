import { Header } from '@/components/common/Header'
import { useRouter } from 'next/router'
import { ProjectDetail } from '@/components/project/ProjectDetail'
import { Main } from '@/components/common/Main'

export default function () {
  const router = useRouter()
  const { id } = router.query
  if (typeof id !== 'string') {
    return (<></>)
  }

  return (
    <>
      <Header />
      <Main>
        <ProjectDetail />
      </Main>
    </>
  )
}
