import { PageTitle } from '@/components/common/PageTitle'
import { TopProjectDashboardItem } from '@/components/project/TopProjectDashboardItem'
import Link from 'next/link'

export const TopProjectDashboard = () => {
  return (
    <section>
      <Link href='/project/configure'>
        <PageTitle title='Project' />
      </Link>
      <TopProjectDashboardItem id='aa' title='aa' />
    </section>
  )
}