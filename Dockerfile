FROM rust:slim
COPY ./target/release/web-app-host ./target/release/web-app-host 
COPY ./wwwroot ./wwwroot 
ENTRYPOINT ["./target/release/web-app-host"]