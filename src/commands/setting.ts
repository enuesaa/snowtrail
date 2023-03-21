import { useQueriesInit } from '@/commands/use'

export const {
  useUpDataQuery,
  useUpDataLazy,
} = useQueriesInit<{}, string>('upData')

export const {
  useDownDataQuery,
  useDownDataLazy,
} = useQueriesInit<{}, string>('downData')
