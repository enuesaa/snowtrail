import { PageTitle } from '@/components/common/PageTitle'
import { useRunLazy } from '@/commands/script'
import { useForm } from 'react-hook-form'
import { TextInput } from '@/components/common/TextInput'
import { css, useTheme } from '@emotion/react'

type FormData = {
  run: string;
}
export const ConfigureAdd = () => {
  const theme = useTheme()
  const { data, invoke } = useRunLazy()
  const { register, handleSubmit } = useForm<FormData>()

  const handleInvoke = handleSubmit((data) => {
    invoke({ run: data.run })
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
      <PageTitle title='Script Add' />
      {data?.toString() ?? ''}
      <form onSubmit={handleInvoke} css={styles.form}>
        <TextInput label='' regist={register('run')} />
        <button type='submit'>run</button>
      </form>
    </section>
  )
}
