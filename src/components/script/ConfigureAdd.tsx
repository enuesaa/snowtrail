import { PageTitle } from '@/components/common/PageTitle'
import { useRunLazy } from '@/commands/script'
import { FormEventHandler } from 'react'

export const ConfigureAdd = () => {
  const { data, invoke } = useRunLazy()

  const handleInvoke: FormEventHandler<HTMLFormElement> = (e) => {
    e.preventDefault()
    const formElms = e.currentTarget.elements
    const run = (formElms.namedItem('run') as HTMLInputElement).value ?? ''
    invoke({ req: run })
  }

  return (
    <section>
      <PageTitle title='Script Add' />
      {data?.toString() ?? ''}
      <form onSubmit={handleInvoke}>
        <input type='text' name='run' />
        <button type='submit'></button>
      </form>
    </section>
  )
}
