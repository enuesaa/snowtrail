import { useState, useEffect } from 'react'
import { invoke as invokeTauri, type InvokeArgs } from '@tauri-apps/api/tauri'

export const useQueryInit = <A, R>(cmd: string) => (arg: A) => {
  const [data, setData] = useState<R|null>(null)
  useEffect(() => {
    (async () => {
      const res = await invokeTauri<R>(cmd, arg as InvokeArgs)
      setData(res)
    })()
  }, [])

  return data
}

export const useLazyQueryInit = <A, R>(cmd: string) => () => {
  const [data, setData] = useState<R|null>(null)
  const invoke = async (arg: A) => {
    const res = await invokeTauri<R>(cmd, arg as InvokeArgs)
    setData(res)
  }

  return { data, invoke }
}
