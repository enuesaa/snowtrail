import { Button, Container, Heading } from '@radix-ui/themes'
import { Header } from '@/components/common/Header'
import { useParams } from 'wouter'
import { LogView } from '@/components/logs/LogView'
import { FaArrowLeft } from 'react-icons/fa'
import { Link } from '@/components/common/Link'

export default function Page() {
  const { name } = useParams()
  if (name === undefined) {
    return <></>
  }

  return (
    <>
      <Header />
      <Container>
        <Heading mt='4'>Log {name}</Heading>
        <LogView name={name} />
        <Button variant='outline' asChild>
          <Link href={'/logs'} m='3'>
            <FaArrowLeft /> Logs
          </Link>
        </Button>
      </Container>
    </>
  )
}
