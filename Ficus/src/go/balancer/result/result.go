package result

import "balancer/void"

type Result[T any] struct {
	value *T
	error error
}

func Ok[T any](value *T) Result[T] {
	return Result[T]{value, nil}
}

func Err[T any](err error) Result[T] {
	if err == nil {
		panic("Trying to create Err result with nil error")
	}

	return Result[T]{nil, err}
}

func From[T any](value *T, err error) Result[T] {
	if err != nil && value != nil {
		panic("Can not create a result with both err and value are not nil")
	}

	if err == nil && value == nil {
		panic("Can not create a result with both err and value are nil")
	}

	if err != nil {
		return Err[T](err)
	}

	return Ok(value)
}

func FromErr(err error) Result[void.Void] {
	if err != nil {
		return Err[void.Void](err)
	}

	return Ok(void.Instance)
}

func (this *Result[T]) IsOk() bool {
	return this.value != nil
}

func (this *Result[T]) IsErr() bool {
	return !this.IsOk()
}

func (this *Result[T]) Ok() *T {
	if !this.IsOk() {
		panic("Retrieving Ok when result is err")
	}

	return this.value
}

func (this *Result[T]) Err() error {
	if !this.IsErr() {
		panic("Retrieving Err when the result is ok")
	}

	return this.error
}

func (this *Result[T]) ToTuple() (*T, error) {
	return this.value, this.error
}
