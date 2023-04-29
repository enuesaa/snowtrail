import { useProjectCreateLazy } from '@/commands/poject'
import { PageTitle } from '@/components/common/PageTitle'
import { TextInput } from '@/components/common/TextInput'
import { useStyles } from '@/styles/use'
import { useForm } from 'react-hook-form'

type FormData = {
  name: string
  workdir: string
}
export const Add = () => {
  const { invoke: invokeProjectCreate } = useProjectCreateLazy()
  const { register, handleSubmit } = useForm<FormData>()

  const hanldeCreateProject = handleSubmit((data) => {
    invokeProjectCreate({ data })
  })

  const styles = useStyles((theme) => ({
    form: theme({ around: 'x1' }).css({
      input: theme({ surf: 'sub', size: 'x1', around: 'x1' }).to(),
      button: theme({ surf: 'reverse', size: 'x1', decorate: 'rounded', around: 'x2' })
        .css({
          cursor: 'pointer',
        })
        .to(),
    }),
  }))

  return (
    <section>
      <PageTitle title='projects.{ new }' />
      <form onSubmit={hanldeCreateProject} css={styles.form}>
        <TextInput label='name' regist={register('name')} />
        <TextInput label='workdir' regist={register('workdir')} />
        <button type='submit'>save</button>
      </form>
    </section>
  )
}
