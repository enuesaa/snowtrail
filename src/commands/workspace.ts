import { queriesInit } from '@/commands/use'

export const {
  useGetWorkspaceQuery,
  useGetWorkspaceLazy,
} = queriesInit<{}, {}>('get_workspace')

export const {
  useSetWorkspaceQuery,
  useSetWorkspaceLazy,
} = queriesInit<{}, {}>('set_workspace')
