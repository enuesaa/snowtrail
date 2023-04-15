import { PageTitle } from '@/components/common/PageTitle'
import { useForm } from 'react-hook-form'
import { TextInput } from '@/components/common/TextInput'
import { useProjectCreateLazy } from '@/commands/poject'

type FormData = {
  name: string;
  workdir: string;
}
export const ConfigureAdd = () => {
  const { invoke: invokeProjectCreate } = useProjectCreateLazy()
  const { register, handleSubmit } = useForm<FormData>()

  const hanldeCreateProject = handleSubmit(data => {
    invokeProjectCreate({ data })
  })

  return (
    <section>
      <PageTitle title='Project Add' />
      <form onSubmit={hanldeCreateProject}>
        <TextInput label='name' regist={register('name')} />
        <TextInput label='workdir' regist={register('workdir')} />
        <button type='submit'>save</button>
      </form>
    </section>
  )
}

