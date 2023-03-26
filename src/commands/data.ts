import { queriesInit } from '@/commands/use'

export const {
  useUpDataQuery,
  useUpDataLazy,
} = queriesInit<{}, string>('up_data')

export const {
  useStatusDataQuery,
  useStatusDataLazy,
} = queriesInit<{}, boolean>('status_data')

export const {
  useDownDataQuery,
  useDownDataLazy,
} = queriesInit<{}, string>('down_data')

export const {
  usePutEventQuery,
  usePutEventLazy,
} = queriesInit<{}, string>('put_event')

export const {
  useListEventsQuery,
  useListEventsLazy,
} = queriesInit<{}, string[]>('list_events')
