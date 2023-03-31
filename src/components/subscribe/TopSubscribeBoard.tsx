import { PageTitle } from '@/components/common/PageTitle'
import Link from 'next/link'

export const TopSubscribeBoard = () => {
  return (
    <section>
      <Link href='/subscribe/configure'>
        <PageTitle title='Subscribe' />
      </Link>
    </section>
  )
}