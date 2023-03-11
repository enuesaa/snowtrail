import { useAtom } from 'jotai'
import { invoke as invokeTauri, type InvokeArgs } from '@tauri-apps/api/tauri'
import { type Fetchmeta } from '@/states/type'

export type UseFetchResult<R, A> = {
  data: null | R;
  invoke: (arg: A) => void,
}
export const useFetch = <R, A>(meta: Fetchmeta<R, A>): UseFetchResult<R, A> => {
  const [value, setValue] = useAtom(meta.atom)
  const invoke = async (arg: A) => {
    const res = await invokeTauri(meta.name, arg as InvokeArgs)
    console.log(res)
    setValue(res as R)
  }
  return { data: value, invoke }
}