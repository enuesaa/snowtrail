import { useStyles } from '@/styles/use'
import Link from 'next/link'

type Props = {
  title: string
  id: string
}
export const TopEventDashboardItem = ({ title, id }: Props) => {
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
      <Link href={`/event/items/${id}`}>{title}</Link>
    </li>
  )
}
