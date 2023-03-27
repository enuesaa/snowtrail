import { PageTitle } from '@/components/common/PageTitle'
import { TopProjectDashboardItem } from '@/components/project/TopProjectDashboardItem'
import { css, useTheme } from '@emotion/react'

export const TopProjectDashboard = () => {
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
      <PageTitle title='Project' />
      <TopProjectDashboardItem id='aa' title='aa' />
    </section>
  )
}