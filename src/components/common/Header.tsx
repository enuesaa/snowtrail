import { useStyles } from '../../styles/use'
import { IoMdSnow } from 'react-icons/io'
import { MdOutlineSettings } from 'react-icons/md'

export const Header = () => {
  const styles = useStyles((theme) => ({
    top: theme({ surf: 'main', size: 'x2' }).css({
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
        <a href='/'>
          <IoMdSnow />
          snowtrail
        </a>
      </div>
      <a href='/settings' css={styles.iconlink}>
        <MdOutlineSettings />
      </a>
    </header>
  )
}
