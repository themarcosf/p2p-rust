#!/usr/bin/env bash

# Script for [...] server and tasks.

# Usage instructions for the script
show_help() {
    cat << EOF
Usage: ./run.sh [OPTIONS]

Run the application or perform various tasks.

OPTIONS:
    --help                          Show this help message
    --check                         Check if the application compiles without building (syntax check)
    --build                         Build the application without running
    --release                       Build the application in release mode
    --run                           Build and run the application in development mode
    --shell                         Access the Rust REPL
    --tests                         Run the tests
    --update                        Update project dependencies

EOF
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

    --build)
        echo "Building application..."
        cd /var/app/src/project || { echo "Failure: /var/app/src/project dir does not exist."; exit 10; }
        cargo build
    ;;

    --check)
        echo "Checking application..."
        cd /var/app/src/project || { echo "Failure: /var/app/src/project dir does not exist."; exit 10; }
        cargo check
    ;;

    --release)
        echo "Building application for release..."
        cd /var/app/src/project || { echo "Failure: /var/app/src/project dir does not exist."; exit 10; }
        cargo build --release
    ;;

    --run)
        # export CHAT_P2P_PORT=9999
        # export CHAT_PEER=/ip4/127.0.0.1/tcp/8888
        echo "Starting application..."
        cd /var/app/src/project || { echo "Failure: /var/app/src/project dir does not exist."; exit 10; }
        cargo run
    ;;

    --shell)
        evcxr
    ;;

    --tests)
        echo "To be implemented..."
    ;;

    --update)
        echo "Updating dependencies..."
        cd /var/app/src/project || { echo "Failure: /var/app/src/project dir does not exist."; exit 10; }
        cargo update
    ;;

    *)
        echo "Unknown option: $1"
        show_help
        EXIT_CODE=1
    ;;
esac

# exit with the last exit code
exit $EXIT_CODE
