import { PageTitle } from '@/components/common/PageTitle'
import { ListItem } from '@/components/subscribe/ListItem'
import Link from 'next/link'
import { useStyles } from '@/styles/use'

export const List = () => {
  const styles = useStyles(theme => ({
    addLink: theme({ surf: 'sub', decorate: 'rounded',around: 'x2' }),
  }))

  return (
    <section>
      <PageTitle title='Subscribe' />
      <Link href='/subscribes?create' css={styles.addLink}>add</Link>
      <ListItem title='aaa' id='aaa' />
    </section>
  )
}
