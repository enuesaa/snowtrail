import { queriesInit } from '@/commands/use'

export const {
  useUpDataQuery,
  useUpDataLazy,
} = queriesInit<{}, string>('upData')

export const {
  useDownDataQuery,
  useDownDataLazy,
} = queriesInit<{}, string>('downData')
