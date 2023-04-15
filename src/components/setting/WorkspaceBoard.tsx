import { useGetWorkspaceQuery, useSetWorkspaceLazy } from '@/commands/workspace'
import { PageTitle } from '@/components/common/PageTitle'
import { useForm } from 'react-hook-form'
import { TextInput } from '@/components/common/TextInput'
import { useStyles } from '@/styles/use'

type FormData = {
  dir: string
  auto_add_new_project: boolean
}
export const WorkspaceBoard = () => {
  const data = useGetWorkspaceQuery({})
  const { invoke: invokePutWorkspace } = useSetWorkspaceLazy()
  const { register, handleSubmit } = useForm<FormData>()

  const handlePutSetting = handleSubmit((data) => {
    invokePutWorkspace({ data })
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
      <PageTitle title='workspace' />
      <form css={styles.form} onSubmit={handlePutSetting}>
        <TextInput label='dir' regist={register('dir')} />
        <TextInput label='auto_add_new_project' regist={register('auto_add_new_project')} />
        <button>save</button>
      </form>
    </section>
  )
}
