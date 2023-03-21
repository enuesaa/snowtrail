import { usePutEventLazy } from '@/commands/setting'
import { useTheme } from '@emotion/react'
import { MouseEventHandler } from 'react'

export const Event = () => {
  const theme = useTheme()

  const { invoke } = usePutEventLazy()

  const handlePutEvent: MouseEventHandler<HTMLButtonElement> = (e) => {
    e.preventDefault()
    invoke({})
  }

  return (
    <>
      <button onClick={handlePutEvent}>putEvent</button>
    </>
  )
}