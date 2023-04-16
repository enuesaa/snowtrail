import { PageTitle } from '@/components/common/PageTitle'
import { useForm } from 'react-hook-form'
import { TextInput } from '@/components/common/TextInput'
import { useStyles } from '@/styles/use'

type FormData = {
  name: string
  description: string
  rule: string
  script_id: string
  mapping_path: string
  mapping_expression: string
}
export const Detail = () => {
  // const { invoke: invokeSetWorkspace } = useSetWorkspaceLazy()
  const { register, handleSubmit } = useForm<FormData>()

  const handleUpdateSubscribe = handleSubmit((data) => {
    // invokeUpdateSubscribe({ data })
  })

  const styles = useStyles((theme) => ({
    form: theme().css({
      input: {
        // ...theme.input,
        background: 'rgba(255,255,255,0.1)',
        padding: '5px 7px',
        borderRadius: '5px',
        // color: theme.color.main,
        margin: '5px 0 20px 0',
        // fontSize: theme.fontSize.large,
      },
      button: {
        // ...theme.input,
        background: 'rgna(0,0,0,0.1)',
        padding: '5px',
        borderRadius: '5px',
      },
    }),
  }))

  return (
    <section>
      <PageTitle title='Subscribe aa' />
      <form css={styles.form} onSubmit={handleUpdateSubscribe}>
        <TextInput label='name' regist={register('name')} />
        <TextInput label='description' regist={register('description')} />
        <TextInput label='rule' regist={register('rule')} />
        <TextInput label='script_id' regist={register('script_id')} />
        <TextInput label='mapping_path' regist={register('mapping_path')} />
        <TextInput label='mapping_expression' regist={register('mapping_expression')} />
        <button type='submit'>save</button>
      </form>
    </section>
  )
}
