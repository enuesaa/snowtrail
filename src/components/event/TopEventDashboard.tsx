import { PageTitle } from '@/components/common/PageTitle'
import { css, useTheme } from '@emotion/react'
import { TopEventDashboardItem } from '@/components/event/TopEventDashboardItem'

export const TopEventDashboard = () => {
  const theme = useTheme()

  const styles = {
    list: css({
      listStyleType: 'none',
      padding: '0',
    }),
  }

  return (
    <section>
      <PageTitle title='Events' />
      <ul css={styles.list}>
        <TopEventDashboardItem title='aa' id='aa' />
      </ul>
    </section>
  )
}