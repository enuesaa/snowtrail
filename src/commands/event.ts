import { queriesInit } from '@/commands/use'

export type EventPublishKvSchema = {
  name: string;
  value: string;
}
export type EventPublishSchema = {
  name: string;
  kvs: EventPublishKvSchema[],
}
export const {
  useEventPublishQuery,
  useEventPublishLazy,
} = queriesInit<{data: EventPublishSchema}, {}>('event_publish')

export const {
  useEventListQuery,
  useEventListLazy,
} = queriesInit<{}, EventPublishSchema[]>('event_list')
