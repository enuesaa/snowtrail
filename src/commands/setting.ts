import { queriesInit } from '@/commands/use'

export const {
  useStatusQuery,
  useStatusLazy,
} = queriesInit<{}, string>('status')
