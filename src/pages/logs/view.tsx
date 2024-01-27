import { Button, Container, Heading, Link } from '@radix-ui/themes'
import { Header } from '@/components/common/Header'
import { useParams } from 'react-router-dom'
import { LogView } from '@/components/logs/LogView'
import { FaArrowLeft } from 'react-icons/fa'

export default function Page() {
  const { name } = useParams()
  if (name === undefined) {
    return (<></>)
  }

  return (
    <>
      <Header />
      <Container>
        <Button variant='outline' asChild>
          <Link href={'/logs'}>
            <FaArrowLeft /> Logs
          </Link>
        </Button>
        <Heading mt='4'>Log {name}</Heading>
        <LogView name={name} />
      </Container>
    </>
  )
}
