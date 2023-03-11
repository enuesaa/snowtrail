import { MouseEventHandler } from 'react'
import { useFetch } from '@/states/usefetch'
import { useTheme, css } from '@emotion/react'
import { startSurrealMeta, endSurrealMeta, recordMeta } from '../../states/surreal'
import Link from 'next/link'

export const Board = () => {
  const theme = useTheme()

  const { data, invoke: invokeStartSurreal } = useFetch(startSurrealMeta)
  const { invoke: invokeEndSurreal } = useFetch(endSurrealMeta)
  const { invoke: invokeRecord } = useFetch(recordMeta)

  const handleStart: MouseEventHandler<HTMLButtonElement> = async (e) => {
    e.preventDefault()
    invokeStartSurreal({})
  }
  const handleEnd: MouseEventHandler<HTMLButtonElement> = async (e) => {
    e.preventDefault()
    invokeEndSurreal({})
  }
  const handleInvokeRecord: MouseEventHandler<HTMLButtonElement> = async (e) => {
    e.preventDefault()
    invokeRecord({})
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
      <button onClick={handleInvokeRecord}>record</button>
      <Link href="http://localhost:8000/">check</Link>
    </div>
  )
}