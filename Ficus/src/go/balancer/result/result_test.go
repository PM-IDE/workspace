package result

import (
	"balancer/void"
	"fmt"
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestOk(t *testing.T) {
	res := Ok(void.Instance)
	assertOkResult(t, &res, void.Instance)
}

func assertOkResult[T any](t *testing.T, res *Result[T], value *T) {
	assert.Panics(t, func() { res.Err() })
	assert.Nil(t, res.error)
	assert.NotNil(t, res.value)
	assert.Equal(t, res.Ok(), void.Instance)
}

func assertErrResult[T any](t *testing.T, res *Result[T], err error) {
	assert.NotNil(t, res.error)
	assert.Nil(t, res.value)
	assert.Panics(t, func() { res.Ok() })
	assert.Equal(t, res.Err(), err)
}

func TestErr(t *testing.T) {
	err := fmt.Errorf("xd")
	res := Err[void.Void](err)

	assertErrResult(t, &res, err)
}

func TestFromErr(t *testing.T) {
	err := fmt.Errorf("xd")
	res := FromErr(err)
	assertErrResult(t, &res, err)

	res = FromErr(nil)
	assertOkResult(t, &res, void.Instance)
}

func TestFromErrAndValue(t *testing.T) {
	err := fmt.Errorf("xd")
	res := From[void.Void](nil, err)
	assertErrResult(t, &res, err)

	res = From(void.Instance, nil)
	assertOkResult(t, &res, void.Instance)

	assert.Panics(t, func() { From[void.Void](nil, nil) })
	assert.Panics(t, func() { From[void.Void](void.Instance, err) })
}
