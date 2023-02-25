import { useRef, MouseEventHandler } from 'react'
import { FeedItem } from './FeedItem'
import { useFetch } from '../../states/usefetch'
import { messageMeta } from '../../states/feed'

export const FeedBoard = () => {
  const { data, invoke } = useFetch(messageMeta)
  const urlInput = useRef<HTMLInputElement>(null)

  const handleClick: MouseEventHandler<HTMLButtonElement> = async (e) => {
    e.preventDefault()
    invoke({ url: urlInput.current?.value ?? '' })
  }

  return (
    <div style={{ color: '#fafafa' }}>
      <input ref={urlInput} />
      <button onClick={handleClick}>Greet</button>
      <h2>{data?.title}</h2>
      {data?.items.map((m,i) => (
        <FeedItem key={i} url={m.url} title={m.title} />
      ))}
    </div>
  )
}