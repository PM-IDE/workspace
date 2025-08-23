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
)

type Container struct {
	BackendService       *services.BackendServiceServer
	ContextValuesService *services.ContextValuesServiceServer
}

func NewContainer(backendService *services.BackendServiceServer, contextValueService *services.ContextValuesServiceServer) *Container {
	return &Container{
		BackendService:       backendService,
		ContextValuesService: contextValueService,
	}
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
	)

	return &Container{}
}
