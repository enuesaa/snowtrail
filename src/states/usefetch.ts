import { useAtom, type PrimitiveAtom } from 'jotai'
import { invoke as invokeTauri, type InvokeArgs } from '@tauri-apps/api/tauri'
import { type FetchAtom } from './type'

export type UseFetchResult<R, A> = {
  data: null | R;
  invoke: (arg: A) => void,
}
export const useFetch = <R, A>(state: PrimitiveAtom<FetchAtom<R, A>>): UseFetchResult<R, A> => {
  const [value, setValue] = useAtom(state)
  const invoke = async (arg: A) => {
    const res = await invokeTauri(value.name, arg as InvokeArgs)
    setValue({ ...value, res: res as R })
  }
  return { data: value.res, invoke }
}