// Made by Grok
// Prompt: 
// Rewrite TimeSpan class from C# standard library to Typescript with the following changes: 
// _ticks field should be replaced with _nanoseconds readonly field of type BigInt and should 
// represent timespan in nanoseconds. The new typescript TimeSpan class should be based on nanoseconds. 
// Create methods to get total nanoseconds, microseconds, milliseconds, seconds, minutes, hours, days. 
// Create methods to get nanoseconds, microseconds, milliseconds, seconds, minutes, hours, days of the Timespan. 
// toString implementation should return string in a Constant (invariant) format "[d'.']hh':'mm':'ss['.'fffffff]"

export class TimeSpan {
  private readonly _nanoseconds: bigint;

  // Constants for time conversions
  private static NanosecondsPerMicrosecond: bigint = BigInt(1000);
  private static NanosecondsPerMillisecond: bigint = TimeSpan.NanosecondsPerMicrosecond * BigInt(1000);
  private static NanosecondsPerSecond: bigint = TimeSpan.NanosecondsPerMillisecond * BigInt(1000);
  private static NanosecondsPerMinute: bigint = TimeSpan.NanosecondsPerSecond * BigInt(60);
  private static NanosecondsPerHour: bigint = TimeSpan.NanosecondsPerMinute * BigInt(60);
  private static NanosecondsPerDay: bigint = TimeSpan.NanosecondsPerHour * BigInt(24);

  // Constructor with nanoseconds
  constructor(nanoseconds: bigint);
  // Constructor with hours, minutes, seconds
  constructor(hours: number, minutes: number, seconds: number);
  // Constructor with days, hours, minutes, seconds
  constructor(days: number, hours: number, minutes: number, seconds: number);
  // Constructor with days, hours, minutes, seconds, milliseconds
  constructor(days: number, hours: number, minutes: number, seconds: number, milliseconds: number);

  constructor(...args: (number | bigint)[]) {
    if (args.length === 1) {
      this._nanoseconds = BigInt(args[0]);
    } else if (args.length === 3) {
      this._nanoseconds = (BigInt(args[0]) * TimeSpan.NanosecondsPerHour) +
        (BigInt(args[1]) * TimeSpan.NanosecondsPerMinute) +
        (BigInt(args[2]) * TimeSpan.NanosecondsPerSecond);
    } else if (args.length === 4) {
      this._nanoseconds = (BigInt(args[0]) * TimeSpan.NanosecondsPerDay) +
        (BigInt(args[1]) * TimeSpan.NanosecondsPerHour) +
        (BigInt(args[2]) * TimeSpan.NanosecondsPerMinute) +
        (BigInt(args[3]) * TimeSpan.NanosecondsPerSecond);
    } else if (args.length === 5) {
      this._nanoseconds = (BigInt(args[0]) * TimeSpan.NanosecondsPerDay) +
        (BigInt(args[1]) * TimeSpan.NanosecondsPerHour) +
        (BigInt(args[2]) * TimeSpan.NanosecondsPerMinute) +
        (BigInt(args[3]) * TimeSpan.NanosecondsPerSecond) +
        (BigInt(args[4]) * TimeSpan.NanosecondsPerMillisecond);
    } else {
      throw new Error("Invalid number of arguments");
    }
  }

  // Static factory methods
  static fromDays(days: number): TimeSpan {
    return new TimeSpan(BigInt(days) * TimeSpan.NanosecondsPerDay);
  }

  static fromHours(hours: number): TimeSpan {
    return new TimeSpan(BigInt(hours) * TimeSpan.NanosecondsPerHour);
  }

  static fromMinutes(minutes: number): TimeSpan {
    return new TimeSpan(BigInt(minutes) * TimeSpan.NanosecondsPerMinute);
  }

  static fromSeconds(seconds: number): TimeSpan {
    return new TimeSpan(BigInt(seconds) * TimeSpan.NanosecondsPerSecond);
  }

  static fromMilliseconds(milliseconds: number): TimeSpan {
    return new TimeSpan(BigInt(milliseconds) * TimeSpan.NanosecondsPerMillisecond);
  }

  static fromMicroseconds(microseconds: number): TimeSpan {
    return new TimeSpan(BigInt(microseconds) * TimeSpan.NanosecondsPerMicrosecond);
  }

  static fromNanoseconds(nanoseconds: bigint): TimeSpan {
    return new TimeSpan(nanoseconds);
  }

  // Component properties
  get days(): number {
    return Number(this._nanoseconds / TimeSpan.NanosecondsPerDay);
  }

  get hours(): number {
    return Number((this._nanoseconds % TimeSpan.NanosecondsPerDay) / TimeSpan.NanosecondsPerHour);
  }

  get minutes(): number {
    return Number((this._nanoseconds % TimeSpan.NanosecondsPerHour) / TimeSpan.NanosecondsPerMinute);
  }

  get seconds(): number {
    return Number((this._nanoseconds % TimeSpan.NanosecondsPerMinute) / TimeSpan.NanosecondsPerSecond);
  }

  get milliseconds(): number {
    return Number((this._nanoseconds % TimeSpan.NanosecondsPerSecond) / TimeSpan.NanosecondsPerMillisecond);
  }

  get microseconds(): number {
    return Number((this._nanoseconds % TimeSpan.NanosecondsPerMillisecond) / TimeSpan.NanosecondsPerMicrosecond);
  }

  get nanoseconds(): number {
    return Number(this._nanoseconds % TimeSpan.NanosecondsPerMicrosecond);
  }

  // Total properties
  get totalDays(): number {
    return Number(this._nanoseconds) / Number(TimeSpan.NanosecondsPerDay);
  }

  get totalHours(): number {
    return Number(this._nanoseconds) / Number(TimeSpan.NanosecondsPerHour);
  }

  get totalMinutes(): number {
    return Number(this._nanoseconds) / Number(TimeSpan.NanosecondsPerMinute);
  }

  get totalSeconds(): number {
    return Number(this._nanoseconds) / Number(TimeSpan.NanosecondsPerSecond);
  }

  get totalMilliseconds(): number {
    return Number(this._nanoseconds) / Number(TimeSpan.NanosecondsPerMillisecond);
  }

  get totalMicroseconds(): number {
    return Number(this._nanoseconds) / Number(TimeSpan.NanosecondsPerMicrosecond);
  }

  get totalNanoseconds(): bigint {
    return this._nanoseconds;
  }

  // Arithmetic operations
  add(ts: TimeSpan): TimeSpan {
    return new TimeSpan(this._nanoseconds + ts.totalNanoseconds);
  }

  subtract(ts: TimeSpan): TimeSpan {
    return new TimeSpan(this._nanoseconds - ts.totalNanoseconds);
  }

  multiply(factor: number): TimeSpan {
    return new TimeSpan(this._nanoseconds * BigInt(factor));
  }

  divide(divisor: number): TimeSpan {
    if (divisor === 0) {
      throw new Error("Division by zero");
    }
    return new TimeSpan(this._nanoseconds / BigInt(divisor));
  }

  // Comparison methods
  equals(ts: TimeSpan): boolean {
    return this._nanoseconds === ts.totalNanoseconds;
  }

  compareTo(ts: TimeSpan): number {
    return this._nanoseconds < ts.totalNanoseconds ? -1 : this._nanoseconds > ts.totalNanoseconds ? 1 : 0;
  }

  // Static comparison methods
  static compare(t1: TimeSpan, t2: TimeSpan): number {
    return t1.compareTo(t2);
  }

  static equals(t1: TimeSpan, t2: TimeSpan): boolean {
    return t1.equals(t2);
  }

  // Utility methods
  negate(): TimeSpan {
    return new TimeSpan(-this._nanoseconds);
  }

  duration(): TimeSpan {
    return new TimeSpan(this._nanoseconds < 0 ? -this._nanoseconds : this._nanoseconds);
  }

  toString(): string {
    const isNegative = this._nanoseconds < 0;
    const absNanos = isNegative ? -this._nanoseconds : this._nanoseconds;

    const days = Number(absNanos / TimeSpan.NanosecondsPerDay);
    const hours = Number((absNanos % TimeSpan.NanosecondsPerDay) / TimeSpan.NanosecondsPerHour);
    const minutes = Number((absNanos % TimeSpan.NanosecondsPerHour) / TimeSpan.NanosecondsPerMinute);
    const seconds = Number((absNanos % TimeSpan.NanosecondsPerMinute) / TimeSpan.NanosecondsPerSecond);
    const nanoseconds = Number(absNanos % TimeSpan.NanosecondsPerSecond);
    const fractionalSeconds = Math.floor(Number(nanoseconds) / 100).toString().padStart(7, '0');

    let result = '';
    if (days > 0) {
      result += `${days}.`;
    }
    result += `${hours.toString().padStart(2, '0')}:${minutes.toString().padStart(2, '0')}:${seconds.toString().padStart(2, '0')}`;
    if (nanoseconds > 0) {
      result += `.${fractionalSeconds}`;
    }
    if (isNegative) {
      result = `-${result}`;
    }
    return result;
  }

  // Static properties
  static get zero(): TimeSpan {
    return new TimeSpan(BigInt(0));
  }

  static get maxValue(): TimeSpan {
    return new TimeSpan(BigInt(Number.MAX_SAFE_INTEGER) * TimeSpan.NanosecondsPerMillisecond);
  }

  static get minValue(): TimeSpan {
    return new TimeSpan(BigInt(Number.MIN_SAFE_INTEGER) * TimeSpan.NanosecondsPerMillisecond);
  }
}