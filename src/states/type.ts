export type FetchAtom<R, A> = {
  name: string;
  res: null | R;
  arg: A;
}