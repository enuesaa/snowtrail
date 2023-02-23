import { Header } from '../components/common/Header'
import { Main } from '../components/common/Main'
import { invoke } from '@tauri-apps/api/tauri'
import { useRef, useState, MouseEventHandler } from 'react';

export default function TopPage() {
  const [message, setMessage] = useState<string>('')
  const urlInput = useRef<HTMLInputElement>(null)

  const handleClick: MouseEventHandler<HTMLButtonElement> = async (e) => {
    e.preventDefault()
    const url = urlInput.current?.value ?? ''
    const res = await invoke('feed', { url })
    setMessage(res as string)
  }

  return (
    <>
      <Header />
      <Main>
        <div style={{ color: '#fafafa' }}>
          <input ref={urlInput} placeholder="Enter a url..." />
          <button onClick={handleClick}>Greet</button>
          <p>{message}</p>
        </div>
      </Main>
    </>
  )
}
