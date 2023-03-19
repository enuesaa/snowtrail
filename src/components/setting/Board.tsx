import { MouseEventHandler } from 'react'
import { useTheme, css } from '@emotion/react'
import { useEndSurrealLazy, useStartSurrealLazy, useRecordLazy } from '@/commands/main'
import Link from 'next/link'

export const Board = () => {
  const theme = useTheme()

  const { data, invoke: invokeStartSurreal } = useStartSurrealLazy()
  const { invoke: invokeEndSurreal } = useEndSurrealLazy()
  const { invoke: invokeRecord } = useRecordLazy()

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