import { css, useTheme } from '@emotion/react'
import { PageTitle } from '@/components/common/PageTitle'

export const EventPublisher = () => {
  const theme = useTheme()

  const styles = {
    main: css({
      margin: '20px',
      padding: '0 10px 10px 10px',
      color: theme.color.main,
    }),
    form: css({
      'input': theme.input,
      'button': theme.input,
    })
  }

  return (
    <section css={styles.main}>
      <PageTitle title='EventPublisher' />
      <form css={styles.form}>
        <input type='text' defaultValue={'event name'} />
        <h3>values</h3>
        <input type='text' defaultValue={'key'} />
        <input type='text' defaultValue={'value'} />
        <button>publish</button>
      </form>
    </section>
  )
}