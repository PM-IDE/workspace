package tests

import (
	"balancer/result"
	"balancer/void"
	"fmt"
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestOk(t *testing.T) {
	res := result.Ok(void.Instance)
	assertOkResult(t, &res, void.Instance)
}

func assertOkResult[T any](t *testing.T, res *result.Result[T], value *T) {
	assert.Panics(t, func() { res.Err() })
	assert.Equal(t, res.Ok(), void.Instance)
	assert.True(t, res.IsOk())
	assert.False(t, res.IsErr())
}

func assertErrResult[T any](t *testing.T, res *result.Result[T], err error) {
	assert.Panics(t, func() { res.Ok() })
	assert.Equal(t, res.Err(), err)
	assert.True(t, res.IsErr())
	assert.False(t, res.IsOk())
}

func TestErr(t *testing.T) {
	err := fmt.Errorf("xd")
	res := result.Err[void.Void](err)

	assertErrResult(t, &res, err)
}

func TestFromErr(t *testing.T) {
	err := fmt.Errorf("xd")
	res := result.FromErr(err)
	assertErrResult(t, &res, err)

	res = result.FromErr(nil)
	assertOkResult(t, &res, void.Instance)
}

func TestFromErrAndValue(t *testing.T) {
	err := fmt.Errorf("xd")
	res := result.From[void.Void](nil, err)
	assertErrResult(t, &res, err)

	res = result.From(void.Instance, nil)
	assertOkResult(t, &res, void.Instance)

	assert.Panics(t, func() { result.From[void.Void](nil, nil) })
	assert.Panics(t, func() { result.From[void.Void](void.Instance, err) })
}
