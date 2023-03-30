import { Header } from '@/components/common/Header'
import { WorkspaceBoard } from '@/components/setting/WorkspaceBoard'
import { Main } from '@/components/common/Main'

export default function Page() {
  return (
    <>
      <Header />
      <Main>
        <WorkspaceBoard />
      </Main>
    </>
  )
}
