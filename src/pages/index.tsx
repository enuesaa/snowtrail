import Link from 'next/link'
import { Header } from '@/components/common/Header'

export default function TopPage() {
  return (
    <>
      <Header />
      <Link href={'/git'} style={{ color: '#fafafa' }}>git</Link>
      <Link href={'/setting'} style={{ color: '#fafafa' }}>setting</Link>
    </>
  )
}
