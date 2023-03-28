import { css, useTheme } from '@emotion/react'
import { PageTitle } from '@/components/common/PageTitle'
import { useEventPublishLazy } from '@/commands/publish'
import { MouseEventHandler } from 'react'

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
    }),
  }

  const { invoke } = useEventPublishLazy()

  const handlePublish: MouseEventHandler<HTMLButtonElement> = (e) => {
    e.preventDefault()
    invoke({ name: 'aa', value: [] })
  }

  return (
    <section css={styles.main}>
      <PageTitle title='EventPublisher' />
      <form css={styles.form}>
        <input type='text' defaultValue={'event name'} />
        <h3>values</h3>
        <input type='text' defaultValue={'key'} />
        <input type='text' defaultValue={'value'} />
        <button onClick={handlePublish}>publish</button>
      </form>
    </section>
  )
}