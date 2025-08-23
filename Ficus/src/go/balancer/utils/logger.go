package utils

import (
	"balancer/result"

	"github.com/google/uuid"
	"go.uber.org/zap"
)

func CreateLoggerAttachedToActivity(originalLogger *zap.SugaredLogger) result.Result[zap.SugaredLogger] {
	id, err := uuid.NewV7()
	if err != nil {
		return result.Err[zap.SugaredLogger](err)
	}

	return result.Ok(originalLogger.With("activity_id", id))
}
