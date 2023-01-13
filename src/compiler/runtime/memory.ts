import binaryen from 'binaryen';
import { ValueType } from '../value.js';

const HEAP_BASE = /*65537*/ 8 * 1000;

export const insertMemoryRuntime = (m: binaryen.Module) => {
  m.addGlobal('~rt/heapTop', binaryen.i32, true, m.i32.const(HEAP_BASE));
  m.addFunction(
    '~rt/alloc',
    binaryen.createType([/* 0: alloc size */ binaryen.i32]),
    binaryen.i32,
    [/* 1: prevHeapTop */ binaryen.i32],
    m.block(
      null,
      [
        m.global.set(
          '~rt/heapTop',
          m.i32.add(
            m.local.tee(
              1,
              m.global.get('~rt/heapTop', binaryen.i32),
              binaryen.i32,
            ),
            m.local.get(0, binaryen.i32),
          ),
        ),
        m.return(m.local.get(1, binaryen.i32)),
      ],
      binaryen.i32,
    ),
  );
  m.addFunction(
    '~rt/createValue/null',
    binaryen.createType([]),
    binaryen.i32,
    [/* 0: ptr */ binaryen.i32],
    m.block(
      null,
      [
        m.local.set(
          0,
          m.call('~rt/alloc', [m.i32.const(32 / 8)], binaryen.i32),
        ),
        m.i32.store(
          0,
          0,
          m.local.get(0, binaryen.i32),
          m.i32.const(ValueType.Null),
        ),
        m.return(m.local.get(0, binaryen.i32)),
      ],
      binaryen.i32,
    ),
  );
  m.addFunction(
    '~rt/createValue/number',
    binaryen.createType([/* 0: the number value */ binaryen.i32]),
    binaryen.i32,
    [/* 1: ptr */ binaryen.i32],
    m.block(
      null,
      [
        m.local.set(
          1,
          m.call('~rt/alloc', [m.i32.const(32 / 8 + 32 / 8)], binaryen.i32),
        ),
        m.i32.store(
          0,
          0,
          m.local.get(1, binaryen.i32),
          m.i32.const(ValueType.Number),
        ),
        m.i32.store(
          4,
          0,
          m.local.get(1, binaryen.i32),
          m.local.get(0, binaryen.i32 /*f64*/),
        ),
        m.return(m.local.get(1, binaryen.i32)),
      ],
      binaryen.i32,
    ),
  );
  m.addFunction(
    '~rt/createValue/string',
    binaryen.createType([
      /* 0: charlen int */ binaryen.i32,
      /* 1: data ptr */ binaryen.i32,
    ]),
    binaryen.i32,
    [/* 2: ptr */ binaryen.i32],
    m.block(
      null,
      [
        // allocate 12 bytes and store in the ptr
        m.local.set(
          2,
          m.call(
            '~rt/alloc',
            [m.i32.const(/* value type */ 4 + /* len */ 4 + /* data ptr */ 4)],
            binaryen.i32,
          ),
        ),
        m.i32.store(
          0,
          0,
          m.local.get(2, binaryen.i32),
          m.i32.const(ValueType.String),
        ),
        m.i32.store(
          4,
          0,
          m.local.get(2, binaryen.i32),
          m.local.get(0, binaryen.i32),
        ),
        m.i32.store(
          8,
          0,
          m.local.get(2, binaryen.i32),
          m.local.get(1, binaryen.i32),
        ),
        m.return(m.local.get(2, binaryen.i32)),
      ],
      binaryen.i32,
    ),
  );
  m.addFunction(
    '~rt/createValue/closure',
    binaryen.createType([
      /* 0: func id */ binaryen.i32,
      /* 1: env len */ binaryen.i32,
    ]),
    binaryen.i32,
    [/* 2: val ptr */ binaryen.i32],
    m.block(
      null,
      [
        m.local.set(
          2,
          m.call(
            '~rt/alloc',
            [m.i32.add(m.i32.const(4 + 4), m.local.get(1, binaryen.i32))],
            binaryen.i32,
          ),
        ),
        m.i32.store(
          0,
          0,
          m.local.get(2, binaryen.i32),
          m.i32.const(ValueType.Closure),
        ),
        m.i32.store(
          4,
          0,
          m.local.get(2, binaryen.i32),
          m.local.get(1, binaryen.i32),
        ),
        m.return(m.local.get(2, binaryen.i32)),
      ],
      binaryen.i32,
    ),
  );
};

export const stringConstSetters = (
  m: binaryen.Module,
  strConst: string,
  strPtr: number,
) => {
  return strConst
    .split('')
    .map((ch) => m.i32.store(2 * (32 / 8), 0, strPtr, ch.charCodeAt(0)));
};
