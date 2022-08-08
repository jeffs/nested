export class AssertionFailure {
  constructor(got, want) {
    this.got = got;
    this.want = want;
  }
};

export function assertEq(got, want) {
  if (got !== want) {
    throw new AssertionFailure(got, want);
  }
}
