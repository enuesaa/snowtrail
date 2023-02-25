import { invoke } from '@tauri-apps/api/tauri'
import { useRef, useState, MouseEventHandler } from 'react'
import { FeedItem } from './FeedItem'

type MessageItem = {
  title: string;
  url: string;
  descripion: string;
}
type Message = {
  title: string;
  items: MessageItem[];
}
export const FeedBoard = () => {
  const [message, setMessage] = useState<Message>({ title: '', items: [] })
  const urlInput = useRef<HTMLInputElement>(null)

  const handleClick: MouseEventHandler<HTMLButtonElement> = async (e) => {
    e.preventDefault()
    const url = urlInput.current?.value ?? ''
    const res = await invoke('feed', { url })
    setMessage(res as Message)
  }

  return (
    <div style={{ color: '#fafafa' }}>
      <input ref={urlInput} />
      <button onClick={handleClick}>Greet</button>
      <h2>{message.title}</h2>
      {message.items.map((m,i) => (
        <FeedItem key={i} url={m.url} title={m.title} />
      ))}
    </div>
  )
}