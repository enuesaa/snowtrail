import { Dashboard as ProejctDashboard } from '@/components/project/Dashboard'
import { Dashboard as SubscribeDashboard } from '@/components/subscribe/Dashboard'
import { Dashboard as EventDashboard } from '@/components/event/Dashboard'

export default function Page() {
  return (
    <>
      <ProejctDashboard />
      <SubscribeDashboard />
      <EventDashboard />
    </>
  )
}
