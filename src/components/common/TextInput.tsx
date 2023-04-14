import { UseFormRegisterReturn } from 'react-hook-form'

type Props = {
  label: string
  regist: UseFormRegisterReturn<string>
}
export const TextInput = ({ label, regist }: Props) => {
  return (
    <label>
      {label}
      <input type='text' {...regist} />
    </label>
  )
}
