import { css } from '@emotion/react'
import Link from 'next/link'

type Props = {
  title: string;
  url: string;
}
export const FeedItem = ({ title, url }: Props) => {
  const styles = {
    link: css({
      display: 'block',
    }),
  }

  return (
    <Link href={url} css={styles.link}>
      {title}
    </Link>
  )
}