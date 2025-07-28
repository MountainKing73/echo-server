BUILD for arm64 and arm64
docker buildx build --tag mountainking73/myecho-server --push --platform linux/arm64,linux/amd64 .

docker run -p 8080:8080 mountainking73/myecho-server
