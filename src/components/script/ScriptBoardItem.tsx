import { css, useTheme } from '@emotion/react'
import Link from 'next/link'

type Props = {
  title: string;
  id: string;
}
export const ScriptBoardItem = ({ title, id }: Props) => {
  const theme = useTheme()

  const styles = {
    card: css(theme.linkCard),
  }

  return (
    <Link href={`/scripts/${id}`} css={styles.card}>
      {title}
    </Link>
  )
}