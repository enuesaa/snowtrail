import { MouseEventHandler } from 'react'
import { useGitHistoriesLazy, useGreetLazy, useAddEventLazy, useLocalfilesLazy } from '@/commands/main'
import { useTheme, css } from '@emotion/react'

export const Board = () => {
  const theme = useTheme()
  const { data, invoke } = useLocalfilesLazy()
  const { data: gitHistories, invoke: invokeGitHistories } = useGitHistoriesLazy()
  const { data: greetData, invoke: invokeGreet } = useGreetLazy()
  const { invoke: invokeAddEvent } = useAddEventLazy()

  const handleClick: MouseEventHandler<HTMLButtonElement> = async (e) => {
    e.preventDefault()
    invoke({})
  }
  const handleGitHistories: MouseEventHandler<HTMLButtonElement> = async (e) => {
    e.preventDefault()
    invokeGitHistories({})
  }
  const handleGreet: MouseEventHandler<HTMLButtonElement> = async (e) => {
    e.preventDefault()
    invokeGreet({ name: 'aaa' })
  }
  const handleAddEvent: MouseEventHandler<HTMLButtonElement> = async (e) => {
    e.preventDefault()
    invokeAddEvent({})
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
      {gitHistories?.items.map((v, i) => (
        <div key={i}>{v.hash}</div>
      ))}
      {greetData}
      <button onClick={handleClick}>aa</button>
      <button onClick={handleGitHistories}>gitHistories</button>
      <button onClick={handleGreet}>greet</button>
      <button onClick={handleAddEvent}>event</button>
    </div>
  )
}