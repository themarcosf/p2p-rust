#!/usr/bin/env bash

# Script for [...] server and tasks.

# Usage instructions for the script
show_help() {
    cat << EOF
Usage: ./run.sh [OPTIONS] [ARGS...]

Run the [...] server or perform various tasks.

This script automatically loads environment variables from a .env file
located at the root of the project. Make sure the required variables
are defined there before running any command.

OPTIONS:
    --help                          Show this help message
    --run                           Run the application
    --tests                         Run the tests using pytest
    --shell                         Access the IPython shell

[ARGS...]:
    Any additional arguments will be forwarded to the relevant subprocesses. Examples:
    ./run.sh --run --tickers AAPL,MSFT --initial-cash 50000 --show-reasoning

ENVIRONMENT VARIABLES:
    API_KEY                         Required. The api key for [...]
    SANDBOX_MODE                    Optional. The sandbox mode (1 or 0). Default is 0.
EOF
}

# Load environment variables from .env file and validate them
function source_and_validate_env() {
    if [[ -f .env ]]; then
        set -a
        # shellcheck disable=SC1091
        source .env
        set +a
    else
        echo "Error: .env file not found at project root."
        exit 1
    fi

    REQUIRED_VARS=(
        API_KEY
    )

    if [[ "${SANDBOX_MODE}" == 1 ]]; then
        REQUIRED_VARS+=()
    fi

    for var in "${REQUIRED_VARS[@]}"; do
        if [[ -z "${!var}" ]]; then
            echo "Error: $var is not set."
            exit 1
        fi
    done

    echo "Environment variables loaded successfully."
}

# Load required environment variables and set up the environment
function run_environment_setup(){
    if [[ ${PROFILE} != 'PROD' ]]; then
        source_and_validate_env
    fi
}

# Run application
function run_application(){
    echo "Starting application..."
    python -m src.main "$@"
}

# Change to the project root directory and handle failure
cd /var/app || { echo "Failure: /var/app dir does not exist."; exit 10; }

# Parse command line arguments
EXIT_CODE=0

# shellcheck disable=SC1009
case $1 in
    --help)
        show_help
    ;;

    --run)
        shift
        run_environment_setup
        run_application "$@"
    ;;

    --tests)
        echo "To be implemented..."
    ;;

    --shell)
        run_environment_setup
        /usr/local/bin/ipython
    ;;

    *)
        echo "Unknown option: $1"
        show_help
        EXIT_CODE=1
    ;;
esac

# exit with the last exit code
exit $EXIT_CODE
