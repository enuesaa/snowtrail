import { PageTitle } from '@/components/common/PageTitle'
import { useStyles } from '@/styles/use'
import Link from 'next/link'
import { FaPlus } from 'react-icons/fa'

export const List = () => {
  const styles = useStyles((theme) => ({
    addLink: theme({ size: 'x1' }),
    item: theme({ surf: 'sub', decorate: 'rounded', around: 'x2' }),
  }))

  return (
    <section>
      <PageTitle title='Subscribe'>
        <Link href='/subscribes?create' css={styles.addLink}>
          <FaPlus />
        </Link>
      </PageTitle>
      <Link href={`/subscribes/aaa`} css={styles.item}>
        aaa
      </Link>
    </section>
  )
}
