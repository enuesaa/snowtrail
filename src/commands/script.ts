import { queriesInit } from '@/commands/use'

export const {
  useRunQuery,
  useRunLazy,
} = queriesInit<{req: string}, String>('run')
