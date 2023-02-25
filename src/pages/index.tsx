import { Header } from '../components/common/Header'
import { Main } from '../components/common/Main'
import { invoke } from '@tauri-apps/api/tauri'
import { useRef, useState, MouseEventHandler } from 'react';

type MessageItem = {
  title: string;
  url: string;
}
type Message = {
  title: string;
  items: MessageItem[];
}
export default function TopPage() {
  const [message, setMessage] = useState<Message>({ title: '', items: [] })
  const urlInput = useRef<HTMLInputElement>(null)

  const handleClick: MouseEventHandler<HTMLButtonElement> = async (e) => {
    e.preventDefault()
    const url = urlInput.current?.value ?? ''
    const res = await invoke('feed', { url })
    setMessage(res as Message)
  }

  return (
    <>
      <Header />
      <Main>
        <div style={{ color: '#fafafa' }}>
          <input ref={urlInput} placeholder="Enter a url..." />
          <button onClick={handleClick}>Greet</button>
          <h2>{message.title}</h2>
          {message.items.map((m,i) => (
            <a key={i} href={m.url}>{m.title}</a>
          ))}
        </div>
      </Main>
    </>
  )
}
