import { useEventListQuery } from '@/commands/event'
import { PageTitle } from '@/components/common/PageTitle'
import { TopEventDashboardItem } from '@/components/event/TopEventDashboardItem'
import { useStyles } from '@/styles/use'

export const TopEventDashboard = () => {
  const styles = useStyles((theme) => ({
    main: theme({ around: 'x1tb' }),
    list: theme().css({
      listStyleType: 'none',
      padding: '0',
    }),
  }))
  const events = useEventListQuery({})

  return (
    <section css={styles.main}>
      <PageTitle title='Events' />
      <ul css={styles.list}>
        {events?.map((v, i) => (
          <TopEventDashboardItem title={v.name} id={v.name} key={i} />
        ))}
      </ul>
    </section>
  )
}
