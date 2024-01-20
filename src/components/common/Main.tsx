import { useStyles } from '../../styles/use'
import { ReactNode } from 'react'
import { useListScripts } from '../../lib/scripts'

type Props = {
  children: ReactNode
}
export const Main = ({ children }: Props) => {
  const {data} = useListScripts()
  const styles = useStyles((theme) => ({
    main: theme({ surf: 'main' }).css({
      margin: '0 15px 15px 15px',
      padding: '0 10px 10px 10px',
    }),
  }))

  return <section css={styles.main}>{children}</section>
}
