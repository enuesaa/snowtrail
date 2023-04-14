import { queriesInit } from '@/commands/use'

export const { useGetWorkspaceQuery, useGetWorkspaceLazy } = queriesInit<{}, {}>('get_workspace')

export const { usePutWorkspaceQuery, usePutWorkspaceLazy } = queriesInit<{}, {}>('put_workspace')
