import { PageTitle } from '@/components/common/PageTitle'
import Link from 'next/link'

export const ProjectScripts = () => {
  return (
    <>
      <PageTitle title='Scripts' />
      <Link href='/project/items/aaa/script/configure/add'>add</Link>
    </>
  )
}
