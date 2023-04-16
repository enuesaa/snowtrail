import { useProjectCreateLazy } from '@/commands/poject'
import { PageTitle } from '@/components/common/PageTitle'
import { TextInput } from '@/components/common/TextInput'
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
