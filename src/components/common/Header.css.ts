import { style } from '@vanilla-extract/css'

export default {
  top: style({
    margin: '0',
    color: '#fafafa',
    fontSize: '20px',
    fontWeight: '600',
    padding: '7px 0',
    boxShadow: '2px 2px 2px rgba(0, 0, 0, 0.7)',
    display: 'flex',
  }),
  title: style({
    margin: '0 0 0 20px',
    color: '#fafafa',
    padding: '2px',
    flex: '1 0 auto',
  }),
  iconlink: style({
    display: 'block',
    flex: '0 0 35px',
    margin: '0 20px 0 0',
    textAlign: 'center',
  }),
}