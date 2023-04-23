import { PageTitle } from '@/components/common/PageTitle'
import { useScriptGetQuery, useScriptDeleteLazy, useScriptRunLazy } from '@/commands/script'
import { MouseEventHandler } from 'react'
import { useStyles } from '@/styles/use';

type Props = {
  name: string;
}
export const ScriptDetail = ({ name }: Props) => {
  const script = useScriptGetQuery({ name })
  const { invoke: invokeDeleteScript } = useScriptDeleteLazy()
  const { invoke: invokeRunScript } = useScriptRunLazy()
  const styles = useStyles(theme => ({
    deleteBtn: theme({ surf: 'reverse', size: 'x1', decorate: 'rounded', around: 'x1' }),
  }))
  if (script === null) {
    return (<></>)
  }
  const handleDeleteScrpt: MouseEventHandler<HTMLButtonElement> = (e) => {
    e.preventDefault()
    invokeDeleteScript({ name })
  }
  const handleRunCommand: MouseEventHandler<HTMLButtonElement> = (e) => {
    e.preventDefault()
    invokeRunScript({ name })
  }

  return (
    <section>
      <PageTitle title={`Script ${script.name}`} />
      <div>
        {script.commands}
      </div>
      <button onClick={handleRunCommand} css={styles.deleteBtn}>run</button>
      <button onClick={handleDeleteScrpt} css={styles.deleteBtn}>delete</button>
    </section>
  )
}
