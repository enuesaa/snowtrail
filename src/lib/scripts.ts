import { useMutation, useQuery, useQueryClient } from 'react-query'
import { invoke } from '@tauri-apps/api/tauri'

export type Script = {
  name: string;
}

export const useListScripts = () => useQuery('listScripts', async (): Promise<Script[]> => {
  const res = await invoke<Script[]>('list_scripts', {})
  return res
})

export const useAddScript = () => {
  const queryclient = useQueryClient()

  return useMutation({
    mutationKey: 'addScript',
    mutationFn: async (script: Script) => {
      await invoke('add_script', {script})
    },
    onSuccess: () => {
      queryclient.invalidateQueries({queryKey: ['listScripts']})
    },
  })
}

export const useRemoveScript = () => {
  const queryclient = useQueryClient()

  return useMutation({
    mutationKey: 'removeScript',
    mutationFn: async (name: string) => {
      await invoke('remove_script', {name})
    },
    onSuccess: () => {
      queryclient.invalidateQueries({queryKey: ['listScripts']})
    },
  })
}
