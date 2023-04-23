import { useProjectListQuery } from '@/commands/poject'
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
  const projects = useProjectListQuery()
  if (projects === null) {
    return <></>
  }

  return (
    <section>
      <PageSubTitle title='Projects'>
        <Link href='/projects?create'>
          <FaPlus />
        </Link>
      </PageSubTitle>
      {projects.map((p) => (
        <Item key={p.name} name={p.name} title={p.name} />
      ))}
    </section>
  )
}
