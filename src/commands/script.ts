import { queriesInit } from '@/commands/use'

export type Script = {
  id?: string | null;
  name: string;
  commands: string[];
  project_name: string;
}

export const { useScriptListQuery, useScriptListLazy } = queriesInit<{ projectName: string }, Script[]>('script_list')

export const { useScriptGetQuery, useScriptGetLazy } = queriesInit<{ name: string }, Script>('script_get')

export const { useScriptCreateQuery, useScriptCreateLazy } = queriesInit<{ data: Script }, void>('script_create')

export const { useScriptDeleteQuery, useScriptDeleteLazy } = queriesInit<{ name: string }, void>('script_delete')

export const {
  useScriptRunQuery,
  useScriptRunLazy,
} = queriesInit<{ name: string }, void>('script_run')
