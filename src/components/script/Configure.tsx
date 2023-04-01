import { PageTitle } from '@/components/common/PageTitle'
import Link from 'next/link'
import { FaPlus } from 'react-icons/fa'

export const Configure = () => {
  return (
    <section>
      <PageTitle title='Script' />
      <Link href='/script/configure/add'><FaPlus /></Link>
    </section>
  )
}