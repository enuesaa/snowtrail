import { Blockquote, Box, Heading, Text } from '@radix-ui/themes'
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
      <Heading mt='4' mb='2'>name</Heading>
      <Text>
        {data.name}
      </Text>
  
      <Heading mt='4' mb='2'>content</Heading>
      <Blockquote>
        {data.content}
      </Blockquote>
      
      <Heading mt='4' mb='2'>time</Heading>
      <Text>
        {data.time}
      </Text>
    </Box>
  )
}
