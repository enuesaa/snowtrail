import { PageTitle } from '@/components/common/PageTitle'
import { css } from '@emotion/react'
import { TopEventDashboardItem } from '@/components/event/TopEventDashboardItem'
import { useEventListQuery } from '@/commands/event'
import { useStatusQuery } from '@/commands/setting'

export const TopEventDashboard = () => {
  // const events = useEventListQuery({})
  const styles = {
    list: css({
      listStyleType: 'none',
      padding: '0',
    }),
  }
  const status = useStatusQuery({})

  return (
    <section>
      <PageTitle title='Events' />
      {status}
      <ul css={styles.list}>
        {/* {
          events?.map((v, i) => (
            <TopEventDashboardItem title={v.name} id={v.name} key={i} />
          ))
        } */}
      </ul>
    </section>
  )
}