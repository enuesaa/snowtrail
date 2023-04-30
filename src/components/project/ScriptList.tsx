import { useScriptListQuery } from '@/commands/script'
import { PageSubTitle } from '@/components/common/PageSubTitle'
import { useStyles } from '@/styles/use'
import Link from 'next/link'
import { FaPlus } from 'react-icons/fa'

type Props = {
  projectId: string
}
export const ScriptList = ({ projectId }: Props) => {
  const scripts = useScriptListQuery({ projectId }) ?? []
  const styles = useStyles((theme) => ({
    addLink: theme({ size: 'x1' }),
    item: theme({ surf: 'sub', decorate: 'rounded', around: 'x2' }),
  }))

  return (
    <>
      <PageSubTitle title='Scripts'>
        <Link href={`/projects/${projectId}/scripts?create`} css={styles.addLink}>
          <FaPlus />
        </Link>
      </PageSubTitle>
      {scripts.map((s, i) => (
        <Link href={`/projects/${projectId}/scripts/${s.id}`} css={styles.item} key={i}>{s.name}</Link>
      ))}
    </>
  )
}
