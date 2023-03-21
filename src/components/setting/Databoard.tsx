import { useUpDataLazy, useDownDataLazy, useStatusDataQuery } from '@/commands/setting'
import { useTheme } from '@emotion/react'
import { MouseEventHandler } from 'react'

export const Databoard = () => {
  const theme = useTheme()

  const status = useStatusDataQuery({})
  const { invoke: invokeUpData } = useUpDataLazy()
  const { invoke: invokeDownData } = useDownDataLazy()

  const handleUpData: MouseEventHandler<HTMLButtonElement> = (e) => {
    e.preventDefault()
    invokeUpData({})
  }
  const handleDownData: MouseEventHandler<HTMLButtonElement> = (e) => {
    e.preventDefault()
    invokeDownData({})
  }

  return (
    <>
      <div style={{ color: '#fafafa' }}>isup: {status ? 'true' : 'false'}</div>
      <button onClick={handleUpData}>up</button>
      <button onClick={handleDownData}>down</button>
    </>
  )
}