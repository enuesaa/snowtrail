import { useGetWorkspaceQuery, useSetWorkspaceLazy } from '@/commands/workspace'
import { PageTitle } from '@/components/common/PageTitle'
import { useForm } from 'react-hook-form'
import { TextInput } from '@/components/common/TextInput'
import { Checkbox } from '@/components/common/Checkbox'

type FormData = {
  path: string
  auto_add_new_project: boolean
}
export const WorkspaceBoard = () => {
  const data = useGetWorkspaceQuery({})
  const { invoke: invokePutWorkspace } = useSetWorkspaceLazy()
  const { register, handleSubmit } = useForm<FormData>()

  if (data === null) {
    return (<></>)
  }

  const handlePutSetting = handleSubmit((data) => {
    console.log('a')
    invokePutWorkspace({ data })
  })

  return (
    <section>
      <PageTitle title='workspace' />
      <form onSubmit={handlePutSetting}>
        <TextInput label='path' regist={register('path')} defaultValue={data.path} />
        <Checkbox label='auto_add_new_project' defaultChecked={data.auto_add_new_project} regist={register('auto_add_new_project')} />
        <button type='submit'>save</button>
      </form>
    </section>
  )
}
