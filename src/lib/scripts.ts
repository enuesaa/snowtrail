import { useQuery } from 'react-query'
import { invoke } from '@tauri-apps/api/tauri'

type Script = {
  id: string;
  name: string;
}

export const useListScripts = () => useQuery('listScripts', async (): Promise<Script[]> => {
  const res = await invoke<Script[]>('list_scripts', {})
  return res
})
