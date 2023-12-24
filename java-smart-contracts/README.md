# Kurtosis Gradle POC

## Overview

This repository provides a Proof of Concept (POC) for using the Kurtosis package to build and test Java smart contracts using Gradle. 

## Prerequisites

Before you begin, ensure you have the following prerequisites installed:

 - Docker
 - Kurtosis CLI

## Getting started

Before starting make sure to clone the repo and switch to this folder.

Step 1: To test, build and optimize the java smart contracts run

```
kurtosis run . --enclave javaenclave
```

Step 2: To download the build folder generated after building the smart contract run

```
kurtosis files download javaenclave contract_artifacts <destination-path>
```

destination-path: The path where the output files should be downloaded