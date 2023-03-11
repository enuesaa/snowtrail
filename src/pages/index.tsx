import Link from 'next/link'
import { Header } from '@/components/common/Header'
import { FeedBoard } from '@/components/feed/FeedBoard'

export default function TopPage() {
  return (
    <>
      <Header />
      <FeedBoard />
      <Link href={'/localfiles'} style={{ color: '#fafafa' }}>localfiles</Link>
      <Link href={'/setting'} style={{ color: '#fafafa' }}>setting</Link>
    </>
  )
}
