import { PageTitle } from '@/components/common/PageTitle'
import { css, useTheme } from '@emotion/react'

export const ScriptDetail = () => {
  const theme = useTheme()

  const styles = {
    main: css({
      margin: '20px',
      padding: '0 10px 10px 10px',
      color: theme.color.main,
    }),
  }

  return (
    <section css={styles.main}>
      <PageTitle title='Script aa' />
    </section>
  )
}