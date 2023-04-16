import { useStyles } from '@/styles/use'
import { ReactNode } from 'react'

type Props = {
  title: string
  children?: ReactNode,
}
export const PageSubTitle = ({ title, children }: Props) => {
  const styles = useStyles((theme) => ({
    main: theme({ around: 'x2tb' }),
    h3: theme({ size: 'x2' }).css({
      display: 'inline-block',
      margin: '0 10px 0 0',
    }),
  }))

  return (
    <div css={styles.main}>
      <h3 css={styles.h3}>
        {title}
      </h3>
      {children}
    </div>
  )
}
