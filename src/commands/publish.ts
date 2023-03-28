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
} = queriesInit<EventPublishRequest, {}>('event_publish')
