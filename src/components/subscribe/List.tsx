import { PageTitle } from '@/components/common/PageTitle'
import { ListItem } from '@/components/subscribe/ListItem'
import Link from 'next/link'

export const List = () => {
  return (
    <section>
      <PageTitle title='Subscribe' />
      <Link href='/subscribes?create'>add</Link>
      <ListItem title='aaa' id='aaa' />
    </section>
  )
}
