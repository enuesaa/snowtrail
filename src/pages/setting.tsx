import { Header } from '@/components/common/Header'
import { Databoard } from '@/components/setting/Databoard'
import { Event } from '@/components/setting/Event'
import { Workspace } from '@/components/setting/Workspace'

export default function TopPage() {
  return (
    <>
      <Header />
      <Databoard />
      <Event />
      <Workspace />
    </>
  )
}
