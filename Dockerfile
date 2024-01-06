FROM rust:1.74.1-slim-bullseye as base

FROM base as builder
WORKDIR /work
COPY . /work
RUN cargo build --release
RUN strip /work/target/release/todos-controller -o /todos-controller

FROM gcr.io/distroless/cc
COPY --from=public.ecr.aws/awsguru/aws-lambda-adapter:0.7.1 /lambda-adapter /opt/extensions/lambda-adapter
COPY --from=builder /todos-controller /
EXPOSE 8080

CMD [ "/todos-controller" ]
