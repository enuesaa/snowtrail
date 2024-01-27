import { Container, Heading } from '@radix-ui/themes'
import { Header } from '../../components/common/Header'
import { LogsTable } from '../../components/logs/LogsTable'

export default function Page() {
  return (
    <>
      <Header />
      <Container>
        <Heading mt='4'>Logs</Heading>
        <LogsTable />
      </Container>
    </>
  )
}
