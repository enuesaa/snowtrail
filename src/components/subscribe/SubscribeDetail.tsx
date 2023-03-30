import { PageTitle } from '@/components/common/PageTitle'
import { css, useTheme } from '@emotion/react'

export const SubscribeDetail = () => {
  const theme = useTheme()

  const styles = {
    form: css({
      'input': theme.input,
      'button': theme.input,
    }),
  }

  return (
    <section>
      <PageTitle title='Subscribe aa' />
      <form css={styles.form}>
        <input type='text' defaultValue={'name'} />
        <input type='text' defaultValue={'description'} />
        <input type='text' defaultValue={'rule'} />
        <input type='text' defaultValue={'script id'} />
        <input type='text' defaultValue={'mapping_path'} />
        <input type='text' defaultValue={'mapping_from'} />
        <button>save</button>
      </form>
    </section>
  )
}
