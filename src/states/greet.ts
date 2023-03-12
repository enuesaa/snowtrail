import { atom } from 'jotai'
import { type Fetchmeta } from '@/states/type'

export type GreetResponse = string;
export type GreetArg = {
  name: string;
}

export const greetMeta: Fetchmeta<GreetResponse, GreetArg> = {
  name: 'greet',
  atom: atom<GreetResponse | null>(null),
}
