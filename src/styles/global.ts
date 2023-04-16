import { css } from '@emotion/react'

export const globalStyle = css`
  html,
  body {
    padding: 0;
    margin: 0;
    font-family: -apple-system, BlinkMacSystemFont, Segoe UI, Roboto, sans-serif;
  }
  a {
    color: inherit;
    text-decoration: none;
  }
  * {
    box-sizing: border-box;
  }
  body {
    background: #1a1a1a;
  }

  input[type='text'],
  textarea {
    background: inherit;
    color: inherit;
    outline: none;
    appearance: none;
    border: none;
    display: block;
  }

  button {
    outline: none;
    appearance: none;
    border: none;
    display: block;
  }
`
