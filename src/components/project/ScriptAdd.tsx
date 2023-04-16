import { useScriptCreateLazy } from '@/commands/script'
import { PageTitle } from '@/components/common/PageTitle'
import { TextInput } from '@/components/common/TextInput'
import { useStyles } from '@/styles/use'
import { useForm } from 'react-hook-form'

type FormData = {
  name: string
  command: string
}
type Props = {
  projectName: string
}
export const ScriptAdd = ({ projectName }: Props) => {
  const { invoke: invokeCreateScript } = useScriptCreateLazy()
  const { register, handleSubmit } = useForm<FormData>()

  const handleInvoke = handleSubmit((data) => {
    const req = {
      name: data.name,
      commands: [data.command],
      project_name: projectName,
    }
    invokeCreateScript({ data: req })
  })

  const styles = useStyles((theme) => ({
    form: theme().css({
      input: {
        background: 'rgba(255,255,255,0.1)',
        padding: '5px 7px',
        borderRadius: '5px',
        margin: '5px 0 20px 0',
      },
      button: {
        background: 'rgna(0,0,0,0.1)',
        padding: '5px',
        borderRadius: '5px',
      },
    }),
  }))

  return (
    <section>
      <PageTitle title='Script Add' />
      <form onSubmit={handleInvoke} css={styles.form}>
        <TextInput label='name' regist={register('name')} />
        <TextInput label='command' regist={register('command')} />
        <button type='submit'>save</button>
      </form>
    </section>
  )
}
