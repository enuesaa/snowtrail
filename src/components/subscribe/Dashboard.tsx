import { PageSubTitle } from '@/components/common/PageSubTitle'
import { useStyles } from '@/styles/use'
import Link from 'next/link'
import { FaPlus } from 'react-icons/fa'

type ItemProps = {
  title: string
  name: string
}
const Item = ({ title, name }: ItemProps) => {
  const styles = useStyles((theme) => ({
    link: theme({ surf: 'sub', size: 'x1', decorate: 'rounded', around: 'x2' }),
  }))

  return (
    <Link href={`/projects/${name}`} css={styles.link}>
      {title}
    </Link>
  )
}

export const Dashboard = () => {
  return (
    <section>
      <PageSubTitle title='Subscribes'>
        <Link href='/subscribes?create'>
          <FaPlus />
        </Link>
      </PageSubTitle>
      <Item name={'a'} title={'b'} />
    </section>
  )
}
