import { Header } from '@/components/common/Header'
import { TopProjectDashboard } from '@/components/project/TopProjectDashboard'
import { TopEventDashboard } from '@/components/event/TopEventDashboard'
import { TopSubscribeBoard } from '@/components/subscribe/TopSubscribeBoard'

export default function () {
  return (
    <>
      <Header />
      <TopProjectDashboard />
      <TopSubscribeBoard />
      <TopEventDashboard />
    </>
  )
}
