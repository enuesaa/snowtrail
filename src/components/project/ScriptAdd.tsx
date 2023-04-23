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
      'input,button': theme({ surf: 'sub', around: 'x1', decorate: 'rounded' }).to(),
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
