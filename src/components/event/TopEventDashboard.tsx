import { PageTitle } from '@/components/common/PageTitle'
import { css, useTheme } from '@emotion/react'
import { TopEventDashboardItem } from '@/components/event/TopEventDashboardItem'

export const TopEventDashboard = () => {
  const theme = useTheme()

  const styles = {
    main: css({
      margin: '20px',
      padding: '0 10px 10px 10px',
      color: theme.color.main,
    }),
    list: css({
      listStyleType: 'none',
      padding: '0',
    }),
  }

  return (
    <section css={styles.main}>
      <PageTitle title='Events' />
      <ul css={styles.list}>
        <TopEventDashboardItem title='aa' id='aa' />
      </ul>
    </section>
  )
}