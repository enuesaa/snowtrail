import Link from 'next/link'
import { css, useTheme } from '@emotion/react'
import { IoMdSnow } from 'react-icons/io'
import { MdOutlineSettings, MdSnowboarding } from 'react-icons/md'

export const Header = () => {
  const theme = useTheme()

  const styles = {
    top: css(theme.box, {
      padding: '7px 0',
      boxShadow: '2px 2px 2px rgba(0, 0, 0, 0.7)',
      display: 'flex',
    }),
    title: css(theme.heading, {
      margin: '0 0 0 20px',
      color: '#fafafa',
      fontSize: theme.fontSize.large,
      padding: '2px',
      flex: '1 0 auto',
      'svg': {
        margin: '0 5px',
        verticalAlign: 'text-top',
      },
    }),
    settinglink: css({
      display: 'block',
      color: theme.color.main,
      flex: '0 0 35px',
      margin: '0 20px 0 0',
      textAlign: 'center',
      fontSize: theme.fontSize.large,
      'svg': {
        verticalAlign: 'middle',
      },
    })
  }

  return (
    <header css={styles.top}>
      <div css={styles.title}>
        <Link href='/'>
          <IoMdSnow />snowtrail
        </Link>
      </div>
      <Link href='/publisher' css={styles.settinglink}>
        <MdSnowboarding />
      </Link>
      <Link href='/setting' css={styles.settinglink}>
        <MdOutlineSettings />
      </Link>
    </header>
  )
}
