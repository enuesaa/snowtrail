import { css } from '@emotion/react'

type Props = {
  name: string
}
export const Title = ({ name }: Props) => {
  const styles = {
    h2: css({
      width: '100%',
      padding: '10px',
      fontWeight: '600',
      color: '#cccccc',
      fontSize: '30px',
    }),
  }

  return <h2 css={styles.h2}>{name}</h2>
}
