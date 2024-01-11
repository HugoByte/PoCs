package main

import (
	"context"
	"fmt"

	"github.com/kurtosis-tech/kurtosis/api/golang/core/kurtosis_core_rpc_api_bindings"
	"github.com/kurtosis-tech/kurtosis/api/golang/core/lib/enclaves"
	"github.com/kurtosis-tech/kurtosis/api/golang/core/lib/starlark_run_config"
	"github.com/kurtosis-tech/kurtosis/api/golang/engine/lib/kurtosis_context"
)

func KurtosisCall(config Node) enclaves.StarlarkRunMultilineOutput {
	fmt.Println("HEllo")
	kurtosisCtx, err := kurtosis_context.NewKurtosisContextFromLocalEngine()
	if err != nil {
		panic(err)
	}

	param := fmt.Sprintf(`{ "relaychain": {
		"name" : %s ,
		"nodes": %v
	} }`, config.Relaychain.Name, config.Relaychain.Nodes)
	
	fmt.Println(param)

	star_config := GetStarlarkRunConfig(param, "/relaychain/relay-chain.star", "start_relay_chains_local")

	ctx := context.Background()
	fmt.Println(kurtosisCtx)
	// encalvectx, err := kurtosisCtx.GetEnclaveContext(ctx, "polkadot_kurto")
	enClave, err := kurtosisCtx.CreateEnclave(ctx, "polkadot-kurto")
	if err != nil {
		panic(err)
	}

	runConfig := GetStarlarkRunConfig("{}", "/package_io/utils.star", "upload_files")
	_, err = enClave.RunStarlarkRemotePackageBlocking(ctx, "github.com/hugobyte/polkadot-kurtosis-package", runConfig)

	if err != nil {
		fmt.Print(err)
	}

	result, errora := enClave.RunStarlarkRemotePackageBlocking(ctx, "github.com/hugobyte/polkadot-kurtosis-package", star_config)

	if errora != nil {
		fmt.Println(errora)
	}

	return result.RunOutput
}

func GetStarlarkRunConfig(params string, relativePathToMainFile string, mainFunctionName string) *starlark_run_config.StarlarkRunConfig {

	starlarkConfig := &starlark_run_config.StarlarkRunConfig{
		RelativePathToMainFile:   relativePathToMainFile,
		MainFunctionName:         mainFunctionName,
		DryRun:                   false,
		SerializedParams:         params,
		Parallelism:              4,
		ExperimentalFeatureFlags: []kurtosis_core_rpc_api_bindings.KurtosisFeatureFlag{},
	}
	return starlarkConfig
}

type Node struct {
	ChainType  string      `json:"chain_type"`
	Relaychain  Relay 	   `json:"relaychain"`
	Para       interface{} `json:"para"`
	Explorer   interface{} `json:"explorer"`
}

type Relay struct {
	Name  string  `json:"name"`
	Nodes string `json:"nodes"`
}

type Nodes struct {
	Name       string `json:"name"`
	NodeType   string `json:"node_type"`
	Prometheus bool   `json:"prometheus"`
}
