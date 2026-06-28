#!/bin/bash

set -euo pipefail

SCRIPT_DIR="$(dirname -- "${BASH_SOURCE[0]}")"

cd "$SCRIPT_DIR"

BUILD_MODE="debug"
TARGETS=("smart-socket-static-app" "smart-socket-dynamic-app")
APP_ARGS=()

while [[ $# -gt 0 ]]; do
	case $1 in
	-r | --release)
		BUILD_MODE="release"
		shift
		;;
	-d | --debug)
		BUILD_MODE="debug"
		shift
		;;
	-b | --bin)
		BINARY_NAME="${2:-}"
		if [[ -z "$BINARY_NAME" ]]; then
			echo "Error: binary name is required"
			exit 1
		fi
		TARGETS=("$BINARY_NAME")
		shift 2
		;;
	--static)
		TARGETS=("smart-socket-static-app")
		shift
		;;
	--dynamic)
		TARGETS=("smart-socket-dynamic-app")
		shift
		;;
	-h | --help)
		echo "Usage: $0 [OPTIONS] [-- APP_ARGS]"
		echo "Options:"
		echo "  -r, --release   Run release binaries"
		echo "  -d, --debug     Run debug binaries by default"
		echo "  -b, --bin NAME  Run selected binary"
		echo "  --static        Run static link demo"
		echo "  --dynamic       Run runtime dynamic link demo"
		echo "  -h, --help      Show this help message"
		echo "Examples:"
		echo "  $0"
		echo "  $0 --static"
		echo "  $0 --dynamic"
		exit 0
		;;
	--)
		shift
		APP_ARGS+=("$@")
		break
		;;
	*)
		APP_ARGS+=("$1")
		shift
		;;
	esac
done

for binary_name in "${TARGETS[@]}"; do
	binary_path="target/$BUILD_MODE/$binary_name"

	if [[ ! -x "$binary_path" ]]; then
		echo "Error: binary not found at $binary_path"
		echo "Run ./build-app.sh first"
		exit 1
	fi

	echo ""
	echo "Running $binary_name"
	"$binary_path" "${APP_ARGS[@]}"
done
