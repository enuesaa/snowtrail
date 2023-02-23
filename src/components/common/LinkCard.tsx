import { css } from '@emotion/react'
import Link from 'next/link'

type Props = {
  href: string
  name: string
}
export const LinkCard = ({ href, name }: Props) => {
  const styles = {
    link: css({
      display: 'inline-block',
      padding: '10px 20px',
      color: '#cccccc',
      fontSize: '20px',
      background: '#333333',
      margin: '10px',
      borderRadius: '10px',
    }),
  }

  return (
    <Link href={href} css={styles.link}>
      {name}
    </Link>
  )
}
