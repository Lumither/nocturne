export type Result<T, E> = Ok<T, E> | Err<T, E>;

class Ok<T, E> {
    readonly value: T;

    constructor(value: T) {
        this.value = value;
    }

    isOk(): this is Ok<T, E> {
        return true;
    }

    isErr(): this is Err<T, E> {
        return false;
    }

    map<U>(fn: (val: T) => U): Result<U, E> {
        return new Ok(fn(this.value));
    }

    mapErr<F>(_fn: (err: E) => F): Result<T, F> {
        return new Ok(this.value);
    }

    unwrap(): T {
        return this.value;
    }

    unwrapOr(_default: T): T {
        return this.value;
    }

    match<U>(handlers: { ok: (val: T) => U, err: (err: E) => U }): U {
        return handlers.ok(this.value);
    }

    andThen<U>(fn: (val: T) => Result<U, E>): Result<U, E> {
        return fn(this.value);
    }
}

class Err<T, E> {
    readonly error: E;

    constructor(error: E) {
        this.error = error;
    }

    isOk(): this is Ok<T, E> {
        return false;
    }

    isErr(): this is Err<T, E> {
        return true;
    }

    map<U>(_fn: (val: T) => U): Result<U, E> {
        return new Err(this.error);
    }

    mapErr<F>(fn: (err: E) => F): Result<T, F> {
        return new Err(fn(this.error));
    }

    unwrap(): T {
        throw new Error('Tried to unwrap Err: ' + this.error);
    }

    unwrapOr(defaultValue: T): T {
        return defaultValue;
    }

    match<U>(handlers: { ok: (val: T) => U, err: (err: E) => U }): U {
        return handlers.err(this.error);
    }

    andThen<U>(_fn: (val: T) => Result<U, E>): Result<U, E> {
        return new Err(this.error);
    }
}

export function ok<T, E = never>(value: T): Result<T, E> {
    return new Ok(value);
}

export function err<T = never, E = unknown>(error: E): Result<T, E> {
    return new Err(error);
}
