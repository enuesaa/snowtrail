import { useScriptListQuery } from '@/commands/script'
import { PageTitle } from '@/components/common/PageTitle'
import Link from 'next/link'

type Props = {
  projectName: string
}
export const ScriptList = ({ projectName }: Props) => {
  const scripts = useScriptListQuery({ projectName }) ?? []

  return (
    <>
      <PageTitle title='Scripts' />
      {scripts.map((s, i) => (
        <div key={i}>{s.name}</div>
      ))}
      <Link href={`/projects/${name}/scripts?create`}>add</Link>
    </>
  )
}
