import { queriesInit } from '@/commands/use'

export const {
  useUpDataQuery,
  useUpDataLazy,
} = queriesInit<{}, string>('upData')

export const {
  useStatusDataQuery,
  useStatusDataLazy,
} = queriesInit<{}, boolean>('statusData')

export const {
  useDownDataQuery,
  useDownDataLazy,
} = queriesInit<{}, string>('downData')

export const {
  usePutEventQuery,
  usePutEventLazy,
} = queriesInit<{}, string>('putEvent')
