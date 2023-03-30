import { Header } from '@/components/common/Header'
import { EventPublisher } from '@/components/publisher/EventPublisher'
import { Main } from '@/components/common/Main'

export default function Page() {
  return (
    <>
      <Header />
      <Main>
        <EventPublisher />
      </Main>
    </>
  )
}
