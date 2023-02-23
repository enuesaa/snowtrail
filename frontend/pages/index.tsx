import { Header } from '../components/common/Header'
import { Main } from '../components/common/Main'
import { invoke } from '@tauri-apps/api/tauri'
import { MouseEventHandler } from 'react';

export default function TopPage() {
  const handleClick: MouseEventHandler<HTMLButtonElement> = (e) => {
    e.preventDefault()
    invoke('greet', { name: 'aa' }).then((a) => { console.log(a) })
  }

  return (
    <>
      <Header />
      <Main>
      <div className="container">
        <h1>Welcome to Tauri!</h1>
        <div className="row">
          <a href="https://tauri.app" target="_blank">
            <img src="/public/assets/tauri.svg" className="logo tauri" alt="Tauri logo" />
          </a>
          <a
            href="https://developer.mozilla.org/en-US/docs/Web/JavaScript"
            target="_blank"
          >
            <img
              src="/public/assets/javascript.svg"
              className="logo vanilla"
              alt="JavaScript logo"
            />
          </a>
        </div>

        <p>Click on the Tauri logo to learn more about the framework</p>

        <div className="row">
          <div>
            <input id="greet-input" placeholder="Enter a name..." />
            <button id="greet-button" type="button" onClick={handleClick}>Greet</button>
          </div>
        </div>

        <p id="greet-msg"></p>
      </div>
      </Main>
    </>
  )
}
