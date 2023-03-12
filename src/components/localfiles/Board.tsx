import { MouseEventHandler } from 'react'
import { useFetch } from '@/states/usefetch'
import { localfilesMeta } from '@/states/localfiles'
import { gitHistoriesMeta } from '@/states/git'
import { greetMeta } from '@/states/greet'
import { useTheme, css } from '@emotion/react'

export const Board = () => {
  const theme = useTheme()
  const { data, invoke } = useFetch(localfilesMeta)
  const { data: gitHistories, invoke: invokeGitHistories } = useFetch(gitHistoriesMeta)
  const { data: greetData, invoke: invokeGreet } = useFetch(greetMeta)

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
    </div>
  )
}