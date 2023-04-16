import { PageTitle } from '@/components/common/PageTitle'
import Link from 'next/link'
import { useScriptListQuery } from '@/commands/script'

type Props = {
  projectName: string;
}
export const ScriptList = ({ projectName }: Props) => {
  const scripts = useScriptListQuery({ projectName }) ?? []

  return (
    <>
      <PageTitle title='Scripts' />
      {scripts.map((s,i) => (
        <div key={i}>{s.name}</div>
      ))}
      <Link href={`/projects/${name}/scripts?create`}>add</Link>
    </>
  )
}
