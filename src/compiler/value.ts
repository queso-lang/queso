import binaryen from 'binaryen';

type ADT<
  name extends string,
  value extends Record<string, any> | any[] | null = null,
> = value extends null ? [name] : [name, value];

export enum ValueType {
  Null = 0,
  Number,
  String,
}

export const createValue = (
  m: binaryen.Module,
  type: ValueType,
  bytes: string[],
) => {
  // return m.i32.store8(m.call('~rt/alloc', [m.i32.const(bytes.length + 1)], binaryen.i32), 0, 0, )
};

// TODO: make this always use the same null in memory?
export const createNullValue = () => {};
