import { useProjectListQuery } from '@/commands/poject'
import { TopProjectDashboardItem } from '@/components/project/TopProjectDashboardItem'

export const TopProjectDashboard = () => {
  const projects = useProjectListQuery()
  console.log(projects)
  if (projects === null) {
    return <></>
  }

  return (
    <section>
      {projects.map((p) => (
        <TopProjectDashboardItem key={p.name} name={p.name} title={p.name} />
      ))}
    </section>
  )
}
