#!/bin/bash

# RUN
# docker run --name rustbot -v $(pwd)/data:/app/data -d $image

# to run without build 
# ./run

# to run with build ( use when new features get added )
# ./run --build 

# to run with migration ( use when database changes are added )
# ./run --migration 

# to run with migration and build
# ./run --build --migration


#default variables
build=false
migrate=false
remove=false
start=false

#Help function
usage() {
    echo "Usage: $0 [OPTIONS]"
    echo "Options:"
    echo " -h, --help       Display this help message"
    echo " -b, --build      Builds the docker container"
    echo " -m, --migration  Runs Migrations."
    echo " -r, --remove     Removes the Old container prior to building"
    echo " -s, --start      Start the Container"
}

handle_options() {
    while [ $# -gt 0 ]; do
        case $1 in
        # display script help
        -h | --help)
        useage
        exit 0
        ;;
        # run docker with build. 
        -b | --build)
        build=true
        ;;
        -r | --remove)
        remove=true
        ;;
        # run docker with migration.
        -m | --migration)
        migrate=true
        ;;
        # start the docker container.
        -s | --start)
        start=true
        ;;
        *)
        echo "Invalid option: $1" >&2
        usage
        exit 1
        ;;
        esac
        shift
    done
}

# main script
handle_options "$@"

# build the docker container.
if [ "$build" = true ]; then
    echo "Building Docker Container"
    if [ "$remove" = true ]; then
        image_sha=$(docker images --no-trunc --quiet rustbot)
        image_sha=${image_sha:7}
        echo "Removing old Container: " $image_sha
        docker image rm $image_sha
    fi
    docker build -t rustbot .
fi


image_sha=$(docker images --no-trunc --quiet rustbot)
image_sha=${image_sha:7}

echo "Running Container: " $image_sha
docker run --name rustbot -v $(pwd)/data:/app/data -d $image_sha

if [ "$migrate" = true ]; then
    echo "Running Migrations"
    # I need to be inside the container.
    docker exec rustbot cargo sqlx migrate run --database-url sqlite:data/rustbot.sqlite --source data/migrations
fi