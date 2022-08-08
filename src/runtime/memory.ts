import binaryen from 'binaryen';
import { ValueType } from '../compiler/value.js';

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
        m.local.set(0, m.call('~rt/alloc', [m.i32.const(0)], binaryen.i32)),
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
    binaryen.createType([/* 0: the number value */ binaryen.f64]),
    binaryen.i32,
    [/* 1: ptr */ binaryen.i32],
    m.block(
      null,
      [
        m.local.set(
          1,
          m.call('~rt/alloc', [m.i32.const(64 / 8)], binaryen.i32),
        ),
        m.i32.store(
          0,
          0,
          m.local.get(1, binaryen.i32),
          m.i32.const(ValueType.Number),
        ),
        m.f64.store(
          4,
          0,
          m.local.get(1, binaryen.i32),
          m.local.get(0, binaryen.f64),
        ),
        m.return(m.local.get(0, binaryen.i32)),
      ],
      binaryen.i32,
    ),
  );
};
