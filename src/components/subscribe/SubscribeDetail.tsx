import { PageTitle } from '@/components/common/PageTitle'
import { useForm } from 'react-hook-form'
import { TextInput } from '@/components/common/TextInput'
import { css, useTheme } from '@emotion/react'

type FormData = {
  name: string;
  description: string;
  rule: string;
  script_id: string;
  mapping_path: string;
  mapping_from: string;
}
export const SubscribeDetail = () => {
  const theme = useTheme()
  // const { invoke: invokeSetWorkspace } = useSetWorkspaceLazy()
  const { register, handleSubmit } = useForm<FormData>()

  const handleUpdateSubscribe = handleSubmit((data) => {
    // invokeUpdateSubscribe({ data })
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
      <PageTitle title='Subscribe aa' />
      <form css={styles.form} onSubmit={handleUpdateSubscribe}>
        <TextInput label='name' regist={register('name')} />
        <TextInput label='description' regist={register('description')} />
        <TextInput label='rule' regist={register('rule')} />
        <TextInput label='script_id' regist={register('script_id')} />
        <TextInput label='mapping_path' regist={register('mapping_path')} />
        <TextInput label='mapping_from' regist={register('mapping_from')} />
        <button type='submit'>save</button>
      </form>
    </section>
  )
}
