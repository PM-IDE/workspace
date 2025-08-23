//go:build wireinject
// +build wireinject

package main

import (
	"balancer/backends"
	"balancer/contextvalues"
	"balancer/executor"
	"balancer/plan"
	"balancer/services"

	"github.com/google/wire"
	"go.uber.org/zap"
)

type Container struct {
	BackendService       *services.BackendServiceServer
	ContextValuesService *services.ContextValuesServiceServer
	Logger               *zap.SugaredLogger
}

func NewContainer(
	backendService *services.BackendServiceServer,
	contextValueService *services.ContextValuesServiceServer,
	logger *zap.SugaredLogger,
) *Container {
	return &Container{
		BackendService:       backendService,
		ContextValuesService: contextValueService,
		Logger:               logger,
	}
}

func NewLogger() *zap.SugaredLogger {
	logger, _ := zap.NewProduction()
	return logger.Sugar()
}

func BuildContainer() *Container {
	wire.Build(
		NewContainer,
		services.NewBackendServiceServer,
		services.NewContextValuesServiceServer,
		plan.NewExecutionPlanner,
		executor.NewPipelineExecutor,
		backends.NewBackendsInfo,
		contextvalues.NewContextValuesStorage,
		NewLogger,
	)

	return &Container{}
}
