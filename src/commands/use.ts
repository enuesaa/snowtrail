import { useState, useEffect } from 'react'
import { invoke as invokeTauri, type InvokeArgs } from '@tauri-apps/api/tauri'

type CommandName = string

const useQueryInit = <A, R>(cmd: CommandName) => (arg: A) => {
  const [data, setData] = useState<R|null>(null)
  useEffect(() => {
    (async () => {
      const res = await invokeTauri<R>(cmd, arg as InvokeArgs)
      setData(res)
    })()
  }, [])

  return data
}

const useLazyInit = <A, R>(cmd: CommandName) => () => {
  const [data, setData] = useState<R|null>(null)
  const invoke = async (arg: A) => {
    const res = await invokeTauri<R>(cmd, arg as InvokeArgs)
    setData(res)
  }

  return { data, invoke }
}

type QueryMap<A, R> = {
  Query: (arg: A) => R;
  Lazy: () => { data: null | R, invoke: (arg: A) => void};
}

export const useQueriesInit = <A, R>(cmd: CommandName) => {
  const queryFn = useQueryInit<A, R>(cmd)
  const lazyFn = useLazyInit<A, R>(cmd)
  const cmdCamel = `${cmd.charAt(0).toUpperCase()}${cmd.slice(1)}`

  return {
    [`use${cmdCamel}Query`]: queryFn,
    [`use${cmdCamel}Lazy`]: lazyFn,
  } as {
    [Q in keyof QueryMap<A, R> as `use${Capitalize<CommandName>}${Q}`]: QueryMap<A, R>[Q];
  }
}
