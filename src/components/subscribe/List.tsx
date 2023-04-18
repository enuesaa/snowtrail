import { PageTitle } from '@/components/common/PageTitle'
import { ListItem } from '@/components/subscribe/ListItem'
import { useStyles } from '@/styles/use'
import Link from 'next/link'

export const List = () => {
  const styles = useStyles((theme) => ({
    addLink: theme({ surf: 'sub', decorate: 'rounded', around: 'x2' }),
  }))

  return (
    <section>
      <PageTitle title='Subscribe' />
      <Link href='/subscribes?create' css={styles.addLink}>
        add
      </Link>
      <ListItem title='aaa' id='aaa' />
    </section>
  )
}
