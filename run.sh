#!/usr/bin/env bash

# Script for [...] server and tasks.

# Usage instructions for the script
show_help() {
    cat << EOF
Usage: ./run.sh [OPTIONS]

Run the application or perform various tasks.

OPTIONS:
    --help                          Show this help message
    --run                           Run the application
    --tests                         Run the tests using cargo
    --shell                         Access the Rust REPL

EOF
}

# Run application
function run_application(){
    echo "Starting application..."
    cd /var/app/src/libp2p_intro || { echo "Failure: /var/app/src/libp2p_intro dir does not exist."; exit 10; }
    cargo run
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
        run_application
    ;;

    --tests)
        echo "To be implemented..."
    ;;

    --shell)
        evcxr
    ;;

    *)
        echo "Unknown option: $1"
        show_help
        EXIT_CODE=1
    ;;
esac

# exit with the last exit code
exit $EXIT_CODE
