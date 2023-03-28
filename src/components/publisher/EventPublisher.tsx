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

  const { invoke } = useEventPublishLazy()

  const handlePublish: MouseEventHandler<HTMLButtonElement> = (e) => {
    e.preventDefault()
    invoke({ event: { name: 'aa', value: [] }})
  }

  return (
    <section css={styles.main}>
      <PageTitle title='EventPublisher' />
      <form css={styles.form}>
        <label htmlFor='eventName'>eventName</label>
        <input type='text' name='eventName' />
        <label htmlFor='valueKey'>key</label>
        <input type='text' name='valueKey' />
        <label htmlFor='valueValue'>value</label>
        <input type='text' name='valueValue' />
        <button onClick={handlePublish}>publish</button>
      </form>
    </section>
  )
}