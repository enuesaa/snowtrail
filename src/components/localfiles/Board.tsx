import { MouseEventHandler } from 'react'
import { useFetch } from '@/states/usefetch'
import { localfilesMeta } from '@/states/localfiles'
import { useTheme, css } from '@emotion/react'

export const Board = () => {
  const theme = useTheme()
  const { data, invoke } = useFetch(localfilesMeta)

  const handleClick: MouseEventHandler<HTMLButtonElement> = async (e) => {
    e.preventDefault()
    invoke({})
  }

  const styles = {
    wrap: css(theme.innerbox, {
      color: theme.color.main,
    }),
  }

  return (
    <div css={styles.wrap}>
      {data?.items.map((v, i) => (
        <div key={i}>{v.name}</div>
      ))}
      <button onClick={handleClick}>aa</button>
    </div>
  )
}