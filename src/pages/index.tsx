import { Container } from '@radix-ui/themes'
import { Header } from '../components/common/Header'
import { ScriptsTable } from '../components/scripts/ScriptsTable'
import { CreateScriptForm } from '../components/scripts/CreateSciprtForm'

export default function Page() {
  return (
    <>
      <Header />
      <Container />
      <CreateScriptForm />
      <ScriptsTable />
    </>
  )
}
