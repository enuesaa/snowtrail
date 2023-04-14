import { queriesInit } from '@/commands/use'

export const { useRunQuery, useRunLazy } = queriesInit<{ run: string }, String>('run')
