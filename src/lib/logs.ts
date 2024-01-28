import { useQuery } from 'react-query'
import { invoke } from '@tauri-apps/api/tauri'

export const useListLogs = () =>
  useQuery('listLogs', async (): Promise<string[]> => {
    const res = await invoke<string[]>('list_logs', {})
    return res
  })

export type LogViewSchema = {
  name: string
  content: string
  time: string
}
export const useGetLog = (name: string) =>
  useQuery(`getLog-${name}`, async (): Promise<LogViewSchema> => {
    const res = await invoke<LogViewSchema>('get_log', { name })
    return res
  })
