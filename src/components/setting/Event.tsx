import { usePutEventLazy, useListEventsQuery } from '@/commands/data'
import { useTheme } from '@emotion/react'
import { MouseEventHandler } from 'react'

export const Event = () => {
  const theme = useTheme()

  const data = useListEventsQuery({})
  const { invoke } = usePutEventLazy()

  const handlePutEvent: MouseEventHandler<HTMLButtonElement> = (e) => {
    e.preventDefault()
    invoke({})
  }

  return (
    <>
      {data?.map(d => {
        return (<>{d}</>)
      })}
      <button onClick={handlePutEvent}>putEvent</button>
    </>
  )
}