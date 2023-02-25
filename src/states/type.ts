import { type PrimitiveAtom } from 'jotai'

export type Fetchmeta<R, A> = {
  name: string;
  atom: PrimitiveAtom<null | R>;
}
