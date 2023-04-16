import { invoke as invokeTauri, type InvokeArgs } from '@tauri-apps/api/tauri'
import { useEffect, useState } from 'react'

type CommandName = string

const queryInit =
  <A, R>(cmd: CommandName) =>
  (arg: A) => {
    const [data, setData] = useState<R | null>(null)
    useEffect(() => {
      ;(async () => {
        const res = await invokeTauri<R>(cmd, arg as InvokeArgs)
        setData(res)
      })()
      // eslint-disable-next-line react-hooks/exhaustive-deps
    }, [])

    return data
  }

const lazyInit =
  <A, R>(cmd: CommandName) =>
  () => {
    const [data, setData] = useState<R | null>(null)
    const invoke = async (arg: A) => {
      const res = await invokeTauri<R>(cmd, arg as InvokeArgs)
      setData(res)
    }

    return { data, invoke }
  }

type QueryMap<A, R> = {
  Query: (arg: A) => R
  Lazy: () => { data: null | R; invoke: (arg: A) => void }
}

type SnakeToCamel<S extends string> = S extends `${infer T}_${infer U}`
  ? `${Capitalize<T>}${SnakeToCamel<U>}`
  : `${Capitalize<S>}`
export const queriesInit = <A, R>(cmd: CommandName) => {
  const queryFn = queryInit<A, R>(cmd)
  const lazyFn = lazyInit<A, R>(cmd)
  const cmdCamel = `${cmd.charAt(0).toUpperCase()}${cmd.slice(1).replace(/_./g, (s) => s.charAt(1).toUpperCase())}`

  return {
    [`use${cmdCamel}Query`]: queryFn,
    [`use${cmdCamel}Lazy`]: lazyFn,
  } as {
    [Q in keyof QueryMap<A, R> as `use${SnakeToCamel<CommandName>}${Q}`]: QueryMap<A, R>[Q]
  }
}
