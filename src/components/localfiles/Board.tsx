import { MouseEventHandler } from 'react'
import { useFetch } from '../../states/usefetch'
import { localfilesMeta } from '../../states/localfiles'
import { runlsMeta } from '../../states/runls'
import { useTheme, css } from '@emotion/react'

export const Board = () => {
  const theme = useTheme()
  const { data, invoke } = useFetch(localfilesMeta)
  const { data: dataRunls, invoke: invokeRunls } = useFetch(runlsMeta)

  const handleClick: MouseEventHandler<HTMLButtonElement> = async (e) => {
    e.preventDefault()
    invoke({})
  }
  const hanldeRunlsClick: MouseEventHandler<HTMLButtonElement> = async (e) => {
    e.preventDefault()
    invokeRunls({})
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
      {dataRunls}
      <button onClick={handleClick}>aa</button>
      <button onClick={hanldeRunlsClick}>runls</button>
    </div>
  )
}