import { css, useTheme } from '@emotion/react'
import Link from 'next/link'

type Props = {
  title: string,
  id: string,
}
export const TopEventDashboardItem = ({ title, id }: Props) => {
  const theme = useTheme()

  const styles = {
    li: css({
      padding: '10px',
      border: 'solid 1px rgba(255,255,255,0.2)',
      a: {
        display: 'block',
        width: '100%',
        height: '100%',
      },
      '&:hover': {
        background: theme.color.sub,
      },
    }),
  }

  return (
    <li css={styles.li}>
      <Link href={`/events/${id}`}>{title}</Link>
    </li>
  )
}
