import { PageTitle } from '@/components/common/PageTitle'
import { useEventGetQuery } from '@/commands/event'

type Props = {
  id: string;
}
export const Detail = ({ id }: Props) => {
  const event = useEventGetQuery({ id })
  if (event === null) {
    return (<></>)
  }

  return (
    <>
      <PageTitle title={`Event ${id}`} />
      <div>id: {id}</div>
      <div>name: {event.name}</div>
      <div>kvs: {event.kvs.map((v,i) => (
        <div key={i}>{v.name}: {v.value}</div>
      ))}</div>
    </>
  )
}