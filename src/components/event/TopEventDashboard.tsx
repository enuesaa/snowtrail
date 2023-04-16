import { useEventListQuery } from '@/commands/event'
import { PageSubTitle } from '@/components/common/PageSubTitle'
import { useStyles } from '@/styles/use'
import Link from 'next/link'

type ItemProps = {
  title: string
  id: string
}
const Item = ({ title, id }: ItemProps) => {
  const styles = useStyles((theme) => ({
    li: theme().css({
      padding: '10px',
      border: 'solid 1px rgba(255,255,255,0.2)',
      a: {
        display: 'block',
        width: '100%',
        height: '100%',
      },
    }),
  }))

  return (
    <li css={styles.li}>
      <Link href={`/events/${id}`}>{title}</Link>
    </li>
  )
}

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
      <PageSubTitle title='Events' />
      <ul css={styles.list}>
        {events?.map((v, i) => (
          <Item title={v.name} id={v.name} key={i} />
        ))}
      </ul>
    </section>
  )
}
