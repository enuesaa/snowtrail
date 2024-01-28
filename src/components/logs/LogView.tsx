import { Box, Heading, Text } from '@radix-ui/themes'
import { useGetLog } from '@/lib/logs'

type Props = {
  name: string
}
export const LogView = ({ name }: Props) => {
  const { data } = useGetLog(name)

  if (data === undefined) {
    return <></>
  }

  return (
    <Box p='3'>
      <Heading mt='3' mb='1'>
        name
      </Heading>
      <Text>{data.name}</Text>
      <Heading mt='3' mb='1'>
        content
      </Heading>
      <Text>{data.content}</Text>
      <Heading mt='3' mb='1'>
        time
      </Heading>
      <Text>{data.time}</Text>
    </Box>
  )
}
