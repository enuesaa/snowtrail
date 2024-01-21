import { globalStyle, style } from '@vanilla-extract/css'

const top = style({
  color: '#fafafa',
  fontSize: '20px',
  lineHeight: '1.5',
  fontWeight: '600',
  padding: '5px 0',
})

globalStyle(`${top} svg`, {
  margin: '0 5px 0 0',
  verticalAlign: 'text-bottom',
})

export default {
  top,
}