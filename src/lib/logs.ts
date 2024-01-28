import { useQuery } from 'react-query'
import { invoke } from '@tauri-apps/api/tauri'

export type LogSchema = {
  name: string
}

export const useListLogs = () =>
  useQuery('listLogs', async (): Promise<LogSchema[]> => {
    const res = await invoke<LogSchema[]>('list_logs', {})
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
