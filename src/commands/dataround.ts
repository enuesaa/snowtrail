import { queriesInit } from '@/commands/use'

export const {
  useDataroundUpQuery,
  useDataroundUpLazy,
} = queriesInit<{}, string>('dataround_up')

export const {
  useDataroundStatusQuery,
  useDataroundStatusLazy,
} = queriesInit<{}, boolean>('dataround_status')

export const {
  useDataroundDownQuery,
  useDataroundDownLazy,
} = queriesInit<{}, string>('dataround_down')
