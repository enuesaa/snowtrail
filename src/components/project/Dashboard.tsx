import { useProjectListQuery } from '@/commands/poject'
import { PageSubTitle } from '@/components/common/PageSubTitle'
import { useStyles } from '@/styles/use'
import Link from 'next/link'
import { FaPlus } from 'react-icons/fa'

type ItemProps = {
  id: string
  name: string
}
const Item = ({ id, name }: ItemProps) => {
  const styles = useStyles((theme) => ({
    link: theme({ surf: 'sub', size: 'x1', decorate: 'rounded', around: 'x2' }),
  }))

  return (
    <Link href={`/projects/${id}`} css={styles.link}>
      {name}
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
      {projects.map(p => 
        typeof p.id === 'string' ? <Item key={p.id} id={p.id} name={p.name} /> : <></>
      )}
    </section>
  )
}
