import binaryen from 'binaryen';
import { ValueType } from '../value.js';

// const createEnumSwitch = (m: binaryen.Module, branches: number[]) => {
//   return m.br
// }

export const insertStd = (m: binaryen.Module) => {
  m.addFunction(
    '~rt/numberToAscii',
    binaryen.createType([/* 0: number */ binaryen.i32]),
    binaryen.i32,
    [],
    m.i32.add(m.local.get(0, binaryen.i32), m.i32.const(30)),
  );
  // m.addFunction(
  //   '~rt/charToAscii',
  //   binaryen.createType(
  //     [
  //       /* 0: char */ binaryen.i32
  //     ],
  //   ),
  //   /* string val ptr */ binaryen.i32,
  //   [],
  //   m.block(null, [], binaryen.i32),
  // );
  m.addFunction(
    '~rt/toString',
    binaryen.createType([/* 0: value ptr */ binaryen.i32]),
    /* string val ptr */ binaryen.i32,
    [/* 1: value type */ binaryen.i32],
    m.block(
      null,
      [
        m.local.set(1, m.i32.load(0, 0, m.local.get(0, binaryen.i32))),
        m.if(
          m.i32.eq(m.local.get(1, binaryen.i32), m.i32.const(ValueType.String)),
          m.local.get(0, binaryen.i32),
          m.i32.const(0),
        ),
      ],
      binaryen.i32,
    ),
  );

  m.addFunction(
    '~rt/print',
    binaryen.createType([/* 0: value ptr */ binaryen.i32]),
    binaryen.none,
    [
      /* 1: str ptr after toString */ binaryen.i32,
      /* 2: *iov_base */ binaryen.i32,
    ],
    m.block(null, [
      m.local.set(1, m.local.get(0, binaryen.i32)),
      // for the iov array
      m.local.set(2, m.call('~rt/alloc', [m.i32.const(4 + 4)], binaryen.i32)),
      // iov_base
      m.i32.store(
        0,
        0,
        m.local.get(2, binaryen.i32),
        m.i32.load(8, 0, m.local.get(1, binaryen.i32)),
      ),
      // iov len
      m.i32.store(
        4,
        0,
        m.local.get(2, binaryen.i32),
        m.i32.load(4, 0, m.local.get(1, binaryen.i32)),
      ),
      m.drop(
        m.call(
          '~rt/fd_write',
          [
            m.i32.const(1),
            m.local.get(2, binaryen.i32),
            m.i32.const(1),
            m.call('~rt/alloc', [m.i32.const(4)], binaryen.i32),
          ],
          binaryen.i32,
        ),
      ),
    ]),
  );
};
