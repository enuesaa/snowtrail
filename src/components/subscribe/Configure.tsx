import { PageTitle } from '@/components/common/PageTitle'
import { SubscribeBoardItem } from '@/components/subscribe/SubscribeBoardItem'
import Link from 'next/link'

export const Configure = () => {
  return (
    <section>
      <PageTitle title='Subscribe' />
      <Link href='/subscribe/configure/add'>add</Link>
      <SubscribeBoardItem title='aaa' id='aaa' />
    </section>
  )
}
