import { Dashboard as EventDashboard } from '@/components/event/Dashboard'
import { Dashboard as ProejctDashboard } from '@/components/project/Dashboard'

export default function Page() {
  return (
    <>
      <ProejctDashboard />
      <EventDashboard />
    </>
  )
}
