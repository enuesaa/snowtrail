import { queriesInit } from '@/commands/use'

export type SubscribeMapingSchema = {
  path: string
  expression: string
}
export type SubscribeSchema = {
  name: string
  description: string
  rule: string[]
  script_id: string
  mapping: SubscribeMapingSchema[]
}

export const { useCreateSubscribeQuery, useCreateSubscribeLazy } = queriesInit<{ data: SubscribeSchema }, {}>(
  'create_subscribe'
)
