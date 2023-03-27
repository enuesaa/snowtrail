import { PageTitle } from '@/components/common/PageTitle'
import { TopSubscribeBoardItem } from '@/components/subscribe/TopSubscribeBoardItem'
import { css, useTheme } from '@emotion/react'

export const TopSubscribeBoard = () => {
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
      <PageTitle title='Subscribe' />
      <TopSubscribeBoardItem id='aa' title='aa' />
    </section>
  )
}