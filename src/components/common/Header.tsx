import { IoMdSnow } from 'react-icons/io'
import { MdOutlineSettings } from 'react-icons/md'
import styles from './Header.css'
import { Link } from '@radix-ui/themes'

export const Header = () => {
  return (
    <header className={styles.top}>
      <div className={styles.title}>
        <Link href='/'>
          <IoMdSnow />
          snowtrail
        </Link>
      </div>
      <Link href='/settings' className={styles.iconlink}>
        <MdOutlineSettings />
      </Link>
    </header>
  )
}
