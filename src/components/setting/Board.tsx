import { MouseEventHandler } from 'react'
import { useFetch } from '../../states/usefetch'
import { useTheme, css } from '@emotion/react'
import { startSurrealMeta, endSurrealMeta } from '../../states/surreal'
import Link from 'next/link'

export const Board = () => {
  const theme = useTheme()

  const { data, invoke: invokeStartSurreal } = useFetch(startSurrealMeta)
  const { invoke: invokeEndSurreal } = useFetch(endSurrealMeta)

  const handleStart: MouseEventHandler<HTMLButtonElement> = async (e) => {
    e.preventDefault()
    invokeStartSurreal({})
  }
  const handleEnd: MouseEventHandler<HTMLButtonElement> = async (e) => {
    e.preventDefault()
    invokeEndSurreal({})
  }

  const styles = {
    wrap: css(theme.innerbox, {
      color: theme.color.main,
    }),
  }

  return (
    <div css={styles.wrap}>
      {data?.status}
      <button onClick={handleStart}>start</button>
      <button onClick={handleEnd}>end</button>
      <Link href="http://localhost:8000/">check</Link>
    </div>
  )
}