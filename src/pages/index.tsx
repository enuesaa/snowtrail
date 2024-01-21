import { Container } from '@radix-ui/themes'
import { Header } from '../components/common/Header'
import { ScriptsTable } from '../components/scripts/ScriptsTable'

export default function Page() {
  return (
    <>
      <Header />
      <Container />
      <ScriptsTable />
    </>
  )
}
