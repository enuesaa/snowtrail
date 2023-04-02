import { PageTitle } from '@/components/common/PageTitle'
import { css } from '@emotion/react'
import { TopEventDashboardItem } from '@/components/event/TopEventDashboardItem'
import { useEventListQuery } from '@/commands/event'

export const TopEventDashboard = () => {
  const styles = {
    list: css({
      listStyleType: 'none',
      padding: '0',
    }),
  }
  const events = useEventListQuery({})

  return (
    <section>
      <PageTitle title='Events' />
      <ul css={styles.list}>
        {
          events?.map((v, i) => (
            <TopEventDashboardItem title={v.name} id={v.name} key={i} />
          ))
        }
      </ul>
    </section>
  )
}