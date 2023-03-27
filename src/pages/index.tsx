import { Header } from '@/components/common/Header'
import { TopProjectDashboard } from '@/components/project/TopProjectDashboard'
import { TopEventDashboard } from '@/components/event/TopEventDashboard'

export default function () {
  return (
    <>
      <Header />
      <TopProjectDashboard />
      <TopEventDashboard />
    </>
  )
}
