import { PositionRange } from '../utils';

const strSplice = (str: String, idx: number, inStr: String) =>
  str.slice(0, idx) + inStr + str.slice(idx);

export class ErrorReporter {
  constructor(public input: string) {}
  report = (msg: string, pos: PositionRange) => {
    // temporary code

    const padToMaxLineNum = (str: string) =>
      str.padStart(('' + lines.length).length);

    let isSameLine = pos.from[0] === pos.to[0];

    const lines = this.input.split(/\r?\n/g);
    // - 1 because line num starts from 1
    const excerpt = lines
      .map((lineStr, i) => ({ lineNo: i + 1, lineStr }))
      .slice(pos.from[0] - 1, pos.to[0] - 1 + 1);

    // add support for filenames later
    let str =
      padToMaxLineNum('') +
      `--> test.queso ` +
      `[${pos.from[0]}:${pos.from[1]}-${pos.to[0]}:${pos.to[1]}]` +
      '\n' +
      padToMaxLineNum('') +
      ' |\n';
    for (const { lineNo, lineStr } of excerpt) {
      str += padToMaxLineNum(`${lineNo}`) + ` | `;

      let tempLineStr = lineStr;
      if (lineNo === pos.from[0]) {
        tempLineStr = strSplice(tempLineStr, pos.from[1] - 1, '\x1b[31m');
      }
      if (lineNo === pos.to[0]) {
        tempLineStr = strSplice(
          tempLineStr,
          pos.to[1] + (isSameLine ? 4 : 0),
          '\x1b[0m',
        );
      }

      str += tempLineStr + '\n';

      str +=
        padToMaxLineNum('') +
        ' | ' +
        ' '.repeat(pos.from[1] - 1) +
        '\x1b[31m' +
        '^'.repeat(pos.to[1] - pos.from[1]) +
        '\x1b[0m\n';
    }

    str += `\x1b[31merror\x1b[0m: ${msg}`;

    console.log('\n' + str + '\n');
  };
}
