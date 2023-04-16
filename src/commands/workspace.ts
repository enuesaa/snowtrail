import { queriesInit } from '@/commands/use'

type WorkspaceSchema = {
  path: string
  auto_add_new_project: boolean
}

export const { useGetWorkspaceQuery, useGetWorkspaceLazy } = queriesInit<{}, WorkspaceSchema>('get_workspace')

export const { useSetWorkspaceQuery, useSetWorkspaceLazy } = queriesInit<{ data: WorkspaceSchema }, {}>('set_workspace')
