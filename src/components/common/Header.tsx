import Link from 'next/link'
import { css, useTheme } from '@emotion/react'
import { IoMdSnow } from 'react-icons/io'

export const Header = () => {
  const theme = useTheme()

  const styles = {
    top: css(theme.box, {
      boxShadow: '2px 2px 2px rgba(0, 0, 0, 0.7)',
    }),
    title: css(theme.heading, {
      color: '#fafafa',
      fontSize: theme.fontSize.large,
      padding: '2px',
    }),
  }

  return (
    <>
      <header css={styles.top}>
        <Link href='/' css={styles.title}><IoMdSnow /></Link>
      </header>
    </>
  )
}
