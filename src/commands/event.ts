import { queriesInit } from '@/commands/use'

export type EventPublishValue = {
  name: string;
  value: string;
}
export type EventPublishRequest = {
  name: string;
  value: EventPublishValue[],
}
export const {
  useEventPublishQuery,
  useEventPublishLazy,
} = queriesInit<{data: EventPublishRequest}, {}>('event_publish')

export const {
  useEventListQuery,
  useEventListLazy,
} = queriesInit<{}, EventPublishRequest[]>('event_list')
