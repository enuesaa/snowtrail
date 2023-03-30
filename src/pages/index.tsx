import { Header } from '@/components/common/Header'
import { TopProjectDashboard } from '@/components/project/TopProjectDashboard'
import { TopEventDashboard } from '@/components/event/TopEventDashboard'
import { TopSubscribeBoard } from '@/components/subscribe/TopSubscribeBoard'
import { Main } from '@/components/common/Main'

export default function Page() {
  return (
    <>
      <Header />
      <Main>
        <TopProjectDashboard />
        <TopSubscribeBoard />
        <TopEventDashboard />
      </Main>
    </>
  )
}
