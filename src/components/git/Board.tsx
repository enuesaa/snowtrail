import { MouseEventHandler } from 'react'
import { useGitHistoriesLazy, useGreetLazy } from '@/commands/main'
import { useTheme, css } from '@emotion/react'

export const Board = () => {
  const theme = useTheme()
  const { data: gitHistories, invoke: invokeGitHistories } = useGitHistoriesLazy()
  const { data: greetData, invoke: invokeGreet } = useGreetLazy()

  const handleGitHistories: MouseEventHandler<HTMLButtonElement> = async (e) => {
    e.preventDefault()
    invokeGitHistories({})
  }
  const handleGreet: MouseEventHandler<HTMLButtonElement> = async (e) => {
    e.preventDefault()
    invokeGreet({ name: 'aaa' })
  }

  const styles = {
    wrap: css(theme.innerbox, {
      color: theme.color.main,
    }),
  }

  return (
    <div css={styles.wrap}>
      {gitHistories?.items.map((v, i) => (
        <div key={i}>{v.hash}</div>
      ))}
      {greetData}
      <button onClick={handleGitHistories}>gitHistories</button>
      <button onClick={handleGreet}>greet</button>
    </div>
  )
}