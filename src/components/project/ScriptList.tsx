import { useScriptListQuery } from '@/commands/script'
import { PageSubTitle } from '@/components/common/PageSubTitle'
import { useStyles } from '@/styles/use'
import Link from 'next/link'
import { FaPlus } from 'react-icons/fa'

type Props = {
  projectName: string
}
export const ScriptList = ({ projectName }: Props) => {
  const scripts = useScriptListQuery({ projectName }) ?? []
  const styles = useStyles((theme) => ({
    addLink: theme({ size: 'x1' }),
    item: theme({ surf: 'sub', decorate: 'rounded', around: 'x2' }),
  }))
  console.log(scripts);

  return (
    <>
      <PageSubTitle title='Scripts'>
        <Link href={`/projects/${projectName}/scripts?create`} css={styles.addLink}>
          <FaPlus />
        </Link>
      </PageSubTitle>
      {scripts.map((s, i) => (
        <Link href={`/projects/${projectName}/scripts/${s.name}`} css={styles.item} key={i}>{s.name}</Link>
      ))}
    </>
  )
}
