import { PageTitle } from '@/components/common/PageTitle'
import Link from 'next/link'

export const Configure = () => {
  return (
    <section>
      <PageTitle title='Script' />
      <Link href='/script/configure/add'>add</Link>
    </section>
  )
}