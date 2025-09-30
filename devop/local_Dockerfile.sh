# ./test_local_Dockerfile.sh

# move where Dockerfile and source code is
cd ..
CONFIGURATION_FILE=configuration.json
VERSION=local-1 #not used
docker build \
    -t portfolio-api:$VERSION \
    --build-arg CONFIGURATION_FILE=$CONFIGURATION_FILE \
    --build-arg VERSION=$VERSION \
    .

docker run -p 8082:3000 --name portfolio-api portfolio-api:$VERSION

# open http://localhost:8082
