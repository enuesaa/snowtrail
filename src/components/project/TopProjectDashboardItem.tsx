import { css, useTheme } from '@emotion/react'
import Link from 'next/link'

type Props = {
  title: string;
  id: string;
}
export const TopProjectDashboardItem = ({ title, id }: Props) => {
  const theme = useTheme()

  const styles = {
    card: css(theme.linkCard),
  }

  return (
    <Link href={`/projects/${id}`} css={styles.card}>
      {title}
    </Link>
  )
}