import { PageTitle } from '@/components/common/PageTitle'
import Link from 'next/link'
import { useScriptListQuery } from '@/commands/script'

type Props = {
  name: string;
}
export const ProjectScripts = ({ name }: Props) => {
  const scripts = useScriptListQuery({ projectName: name }) ?? []

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
