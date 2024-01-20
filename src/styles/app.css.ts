import { globalStyle } from '@vanilla-extract/css'

globalStyle('html, body', {
  padding: 0,
  margin: 0,
})

globalStyle('a', {
  color: 'inherit',
  textDecoration: 'none',
})

globalStyle('*', {
  boxSizing: 'border-box',
})
