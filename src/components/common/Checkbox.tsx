import { UseFormRegisterReturn } from 'react-hook-form'

type Props = {
  label: string;
  defaultChecked?: boolean;
  regist: UseFormRegisterReturn<string>;
}
export const Checkbox = ({ label, regist, defaultChecked }: Props) => {
  return (
    <label>
      {label}
      <input type='checkbox' defaultChecked={defaultChecked ?? false} {...regist} />
    </label>
  )
}
