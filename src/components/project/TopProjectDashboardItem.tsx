import Link from 'next/link'

type Props = {
  title: string
  name: string
}
export const TopProjectDashboardItem = ({ title, name }: Props) => {
  return (
    <Link href={`/project/items/${name}`}>
      {title}
    </Link>
  )
}
