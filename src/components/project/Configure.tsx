import { PageTitle } from '@/components/common/PageTitle'
import Link from 'next/link'

export const Configure = () => {
  return (
    <section>
      <PageTitle title='Project' />
      <Link href='/project/configure/add'>add</Link>
    </section>
  )
}