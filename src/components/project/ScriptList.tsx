import { useScriptListQuery } from '@/commands/script'
import { PageSubTitle } from '@/components/common/PageSubTitle'
import { useStyles } from '@/styles/use'
import Link from 'next/link'

type Props = {
  projectName: string
}
export const ScriptList = ({ projectName }: Props) => {
  const scripts = useScriptListQuery({ projectName }) ?? []
  const styles = useStyles((theme) => ({
    addLink: theme({ surf: 'sub', decorate: 'rounded', around: 'x2' }),
  }))

  return (
    <>
      <PageSubTitle title='Scripts' />
      {scripts.map((s, i) => (
        <div key={i}>{s.name}</div>
      ))}
      <Link href={`/projects/${projectName}/scripts?create`} css={styles.addLink}>
        add
      </Link>
    </>
  )
}
