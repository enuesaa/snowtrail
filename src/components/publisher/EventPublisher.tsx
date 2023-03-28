import { css, useTheme } from '@emotion/react'
import { PageTitle } from '@/components/common/PageTitle'
import { useEventPublishLazy } from '@/commands/event'
import { FormEventHandler } from 'react'

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

  const handlePublish: FormEventHandler<HTMLFormElement> = (e) => {
    e.preventDefault()
    const formElms = e.currentTarget.elements
    const name = (formElms.namedItem('eventName') as HTMLInputElement).value ?? ''
    const key = (formElms.namedItem('valueKey') as HTMLInputElement).value ?? ''
    const value = (formElms.namedItem('valueValue') as HTMLInputElement).value ?? ''
    invoke({ event: { name, value: [{ name: key, value }] }})
  }

  return (
    <section css={styles.main}>
      <PageTitle title='EventPublisher' />
      <form css={styles.form} onSubmit={handlePublish}>
        <label htmlFor='eventName'>eventName</label>
        <input type='text' name='eventName' />
        <label htmlFor='valueKey'>key</label>
        <input type='text' name='valueKey' />
        <label htmlFor='valueValue'>value</label>
        <input type='text' name='valueValue' />
        <button type='submit'>publish</button>
      </form>
    </section>
  )
}