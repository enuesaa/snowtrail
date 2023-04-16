import { PageTitle } from '@/components/common/PageTitle'
import Link from 'next/link'

export const List = () => {
  return (
    <section>
      <PageTitle title='Project' />
      <Link href='/projects?create'>add</Link>
    </section>
  )
}
