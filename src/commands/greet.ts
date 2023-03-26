import { queriesInit } from '@/commands/use'

export const {
  useGreetQuery,
  useGreetLazy,
} = queriesInit<{ name: string }, string>('greet')
