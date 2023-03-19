import { useRef, MouseEventHandler } from 'react'
import { FeedItem } from './FeedItem'
import { useFeedLazy } from '@/commands/main'
import { useTheme, css } from '@emotion/react'

export const FeedBoard = () => {
  const theme = useTheme()
  const { data, invoke } = useFeedLazy()
  const urlInput = useRef<HTMLInputElement>(null)

  const handleClick: MouseEventHandler<HTMLButtonElement> = async (e) => {
    e.preventDefault()
    invoke({ url: urlInput.current?.value ?? '' })
  }

  const styles = {
    wrap: css(theme.innerbox, {
      color: theme.color.main,
    }),
  }

  return (
    <div css={styles.wrap}>
      <input ref={urlInput} />
      <button onClick={handleClick}>Greet</button>
      <h2>{data?.title}</h2>
      {data?.items.map((m,i) => (
        <FeedItem key={i} url={m.url} title={m.title} />
      ))}
    </div>
  )
}