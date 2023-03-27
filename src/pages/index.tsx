import Link from 'next/link'
import { Header } from '@/components/common/Header'

export default function () {
  return (
    <>
      <Header />
      <Link href={'/setting'} style={{ color: '#fafafa' }}>setting</Link>
      <Link href={'/projects/aa'} style={{ color: '#fafafa' }}>projects</Link>
    </>
  )
}
