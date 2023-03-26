import { useGetWorkspaceQuery } from '@/commands/workspace'

export const Workspace = () => {
  const data = useGetWorkspaceQuery({})
  console.log(data)

  return (
    <></>
  )
}