import { useGetWorkspaceQuery, useSetWorkspaceLazy } from '@/commands/workspace'
import { Checkbox } from '@/components/common/Checkbox'
import { PageSubTitle } from '@/components/common/PageSubTitle'
import { TextInput } from '@/components/common/TextInput'
import { useForm } from 'react-hook-form'
import { useStyles } from '@/styles/use'

type FormData = {
  path: string
  auto_add_new_project: boolean
}
export const WorkspaceBoard = () => {
  const data = useGetWorkspaceQuery({})
  const { invoke: invokePutWorkspace } = useSetWorkspaceLazy()
  const { register, handleSubmit } = useForm<FormData>()

  if (data === null) {
    return <></>
  }

  const handlePutSetting = handleSubmit((data) => {
    invokePutWorkspace({ data })
  })

  const styles = useStyles((theme) => ({
    form: theme({ around: 'x1' }).css({
      input: theme({ surf: 'sub', size: 'x1', around: 'x1' }).to(),
      button: theme({ surf: 'reverse', size: 'x1', decorate: 'rounded', around: 'x2' }).css({
        cursor: 'pointer',
      }).to(),
    }),
  }))

  return (
    <section>
      <PageSubTitle title='workspace' />
      <form onSubmit={handlePutSetting} css={styles.form}>
        <TextInput label='path' regist={register('path')} defaultValue={data.path} />
        <Checkbox
          label='auto add'
          defaultChecked={data.auto_add_new_project}
          regist={register('auto_add_new_project')}
        />
        <button type='submit'>save</button>
      </form>
    </section>
  )
}
