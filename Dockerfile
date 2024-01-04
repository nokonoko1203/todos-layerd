FROM rust:1.74.1-slim-bullseye as base

FROM base as builder
WORKDIR /backend
COPY . .
RUN cargo build --release
RUN strip /backend/target/release/app -o /app

FROM gcr.io/distroless/cc
COPY --from=public.ecr.aws/awsguru/aws-lambda-adapter:0.7.1 /lambda-adapter /opt/extensions/lambda-adapter
COPY --from=builder /app /
EXPOSE 8080

CMD [ "./app" ]
