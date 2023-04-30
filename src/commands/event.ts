import { queriesInit } from '@/commands/use'

export type EventPublishKvSchema = {
  name: string;
  value: string;
}
export type EventPublishSchema = {
  id?: string | null;
  name: string;
  kvs: EventPublishKvSchema[];
}
export const { useEventPublishQuery, useEventPublishLazy } = queriesInit<{ data: EventPublishSchema }, {}>(
  'event_publish'
)

export const { useEventListQuery, useEventListLazy } = queriesInit<{}, EventPublishSchema[]>('event_list')

export const { useEventGetQuery, useEventGetLazy } = queriesInit<{ id: string }, EventPublishSchema>('event_get')
