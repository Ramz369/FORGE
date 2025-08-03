export class Option<T> {
  private constructor(private _value: T | null) {}
  static Some<T>(v: T): Option<T> { return new Option<T>(v); }
  static None<T>(): Option<T> { return new Option<T>(null); }
  isSome(): bool { return this._value !== null; }
}
