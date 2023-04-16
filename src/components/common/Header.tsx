import Link from 'next/link'
import { useStyles } from '@/styles/use'
import { IoMdSnow } from 'react-icons/io'
import { MdOutlineSettings, MdSnowboarding } from 'react-icons/md'

export const Header = () => {
  const styles = useStyles((theme) => ({
    top: theme({ surf: 'main' }).css({
      padding: '7px 0',
      boxShadow: '2px 2px 2px rgba(0, 0, 0, 0.7)',
      display: 'flex',
    }),
    title: theme().css({
      margin: '0 0 0 20px',
      color: '#fafafa',
      padding: '2px',
      flex: '1 0 auto',
      svg: {
        margin: '0 5px',
        verticalAlign: 'text-top',
      },
    }),
    textlink: theme().css({
      display: 'block',
      flex: '0 0 35px',
      margin: '0 20px 0 0',
      padding: '2px',
      textAlign: 'center',
      fontWeight: '600',
    }),
    iconlink: theme().css({
      display: 'block',
      flex: '0 0 35px',
      margin: '0 20px 0 0',
      textAlign: 'center',
      svg: {
        verticalAlign: 'middle',
      },
    }),
  }))

  return (
    <header css={styles.top}>
      <div css={styles.title}>
        <Link href='/'>
          <IoMdSnow />
          snowtrail
        </Link>
      </div>
      <Link href='/projects' css={styles.textlink}>
        Project
      </Link>
      <Link href='/subscribe/configure' css={styles.textlink}>
        Subscribe
      </Link>
      <Link href='/publisher' css={styles.iconlink}>
        <MdSnowboarding />
      </Link>
      <Link href='/setting' css={styles.iconlink}>
        <MdOutlineSettings />
      </Link>
    </header>
  )
}
