import { useGetWorkspaceQuery, usePutWorkspaceLazy } from '@/commands/workspace'
import { PageTitle } from '@/components/common/PageTitle'
import { useForm } from 'react-hook-form'
import { TextInput } from '@/components/common/TextInput'
import { css, useTheme } from '@emotion/react'

type FormData = {
  dir: string;
  auto_add_new_project: boolean;
}
export const WorkspaceBoard = () => {
  const theme = useTheme()
  const data = useGetWorkspaceQuery({})
  const { invoke: invokePutWorkspace } = usePutWorkspaceLazy()
  const { register, handleSubmit } = useForm<FormData>()

  const handlePutSetting = handleSubmit((data) => {
    invokePutWorkspace({ data })
  })

  const styles = {
    form: css({
      'input': { 
        ...theme.input,
        background: 'rgba(255,255,255,0.1)',
        padding: '5px 7px',
        borderRadius: '5px',
        color: theme.color.main,
        margin: '5px 0 20px 0',
        fontSize: theme.fontSize.large,
      },
      'button': {
        ...theme.input,
        background: 'rgna(0,0,0,0.1)',
        padding: '5px',
        borderRadius: '5px',
      },
    }),
  }

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