import { PageTitle } from '@/components/common/PageTitle'
import { useScriptGetQuery, useScriptDeleteLazy } from '@/commands/script'
import { MouseEventHandler } from 'react'
import { useStyles } from '@/styles/use';

type Props = {
  name: string;
}
export const ScriptDetail = ({ name }: Props) => {
  const script = useScriptGetQuery({ name })
  const { invoke: invokeDeleteScript } = useScriptDeleteLazy()
  if (script === null) {
    return (<></>)
  }
  const handleDeleteScrpt: MouseEventHandler<HTMLButtonElement> = (e) => {
    e.preventDefault()
    invokeDeleteScript({ name })
  }
  const styles = useStyles(theme => ({
    deleteBtn: theme({ surf: 'reverse', size: 'x1', decorate: 'rounded', around: 'x1' }),
  }))

  return (
    <section>
      <PageTitle title={`Script ${script.name}`} />
      <div>
        {script.commands}
      </div>
      <button onClick={handleDeleteScrpt} css={styles.deleteBtn}>delete</button>
    </section>
  )
}
