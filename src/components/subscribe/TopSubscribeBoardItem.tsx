import { css, useTheme } from '@emotion/react'
import Link from 'next/link'

type Props = {
  title: string;
  id: string;
}
export const TopSubscribeBoardItem = ({ title, id }: Props) => {
  const theme = useTheme()

  const styles = {
    card: css(theme.linkCard),
  }

  return (
    <Link href={`/subscribes/${id}`} css={styles.card}>
      {title}
    </Link>
  )
}