FROM rust:slim
WORKDIR /app
COPY ./target/release/web-app-host ./web-app-host 
COPY ./wwwroot ./wwwroot 
ENTRYPOINT ["./web-app-host"]
