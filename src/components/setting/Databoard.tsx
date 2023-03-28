import { useDataroundStatusQuery, useDataroundUpLazy, useDataroundDownLazy } from '@/commands/dataround'
import { css, useTheme } from '@emotion/react'
import { MouseEventHandler } from 'react'
import { PageTitle } from '../common/PageTitle'

export const Databoard = () => {
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

  const status = useDataroundStatusQuery({})
  const { invoke: invokeUpData } = useDataroundUpLazy()
  const { invoke: invokeDownData } = useDataroundDownLazy()

  const handleUpData: MouseEventHandler<HTMLButtonElement> = (e) => {
    e.preventDefault()
    invokeUpData({})
  }
  const handleDownData: MouseEventHandler<HTMLButtonElement> = (e) => {
    e.preventDefault()
    invokeDownData({})
  }

  return (
    <section css={styles.main}>
      <PageTitle title='DataBoard' />
      <form css={styles.form}>
        <div>isup {status ? 'true' : 'false'}</div>
        <button onClick={handleUpData}>up</button>
        <button onClick={handleDownData}>down</button>
      </form>
    </section>
  )
}