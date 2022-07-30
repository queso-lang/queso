import { Token } from '../lexer/Token';

type EnvLocal = {
  name: Token;
  depth: number;
};

export type EnvUpvalue = {
  id: number;
  isLocal: boolean;
};

export class Env {
  locals: EnvLocal[] = [];
  upvalues: EnvUpvalue[] = [];
  captured: number[] = [];
  scopeDepth = 0;

  addLocal = (name: Token) => {
    this.locals.push({ name, depth: this.scopeDepth });
    return this.locals.length - 1;
  };

  addUpvalue = (upvalue: EnvUpvalue) => {
    for (const [i, upv] of this.upvalues.entries()) {
      if (upv.id === upvalue.id && upv.isLocal == upvalue.isLocal) return i;
    }
    this.upvalues.push(upvalue);
    return this.upvalues.length - 1;
  };

  getLocalAt = (idx: number) => {
    return this.locals[idx];
  };

  openScope = () => {
    this.scopeDepth += 1;
  };

  closeScope = () => {
    this.scopeDepth -= 1;
  };

  isRedefined = (other: Token) => {
    if (this.locals.length === 0) return false;

    let i = this.locals.length - 1;
    while (true) {
      const local = this.locals[i];

      if (local.depth < this.scopeDepth) break;

      if (local.name.val === other.val) return true;

      if (i <= 0) break;
      i -= 1;
    }
    return false;
  };

  capture = (id: number) => {
    this.captured.push(id);
  };
}
