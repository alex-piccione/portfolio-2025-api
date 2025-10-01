# Test Dockerfile(s)

# Compare different Dockerfile


cd ..
CONFIGURATION_FILE=configuration.json
VERSION=local-1

# Dockerfile
docker build \
    -t portfolio-api:test-Dockerfile_1 \
    --build-arg CONFIGURATION_FILE=$CONFIGURATION_FILE \
    --build-arg VERSION=$VERSION \
    --file Dockerfile \
    .


# Dockerfile_2
docker build \
    -t portfolio-api:test-Dockerfile_2 \
    --build-arg CONFIGURATION_FILE=$CONFIGURATION_FILE \
    --build-arg VERSION=$VERSION \
    --file Dockerfile_2 \
    .

# Dockerfile_3
docker build \
    -t portfolio-api:test-Dockerfile_3 \
    --build-arg CONFIGURATION_FILE=$CONFIGURATION_FILE \
    --build-arg VERSION=$VERSION \
    --file Dockerfile_3 \
    .

docker run -p 8081:3000 --name portfolio-api portfolio-api:test-Dockerfile_1
docker run -p 8082:3000 --name portfolio-api portfolio-api:test-Dockerfile_2
docker run -p 8083:3000 --name portfolio-api portfolio-api:test-Dockerfile_3

