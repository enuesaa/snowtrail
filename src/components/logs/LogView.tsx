import { Box, Text } from '@radix-ui/themes'
import { useGetLog } from '../../lib/logs'

type Props = {
  name: string
}
export const LogView = ({ name }: Props) => {
  const { data } = useGetLog(name)

  if (data === undefined) {
    return (<></>)
  }

  return (
    <Box p='3'>
      <Text>
        {data.content}
      </Text>
    </Box>
  )
}
