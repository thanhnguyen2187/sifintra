FROM ubuntu:24.04 AS builder

# Install NodeJS
RUN apt update -y \
    && apt install -y gpg wget curl build-essential
RUN install -dm 755 /etc/apt/keyrings
RUN wget -qO - https://mise.jdx.dev/gpg-key.pub | gpg --dearmor |  tee /etc/apt/keyrings/mise-archive-keyring.gpg 1> /dev/null
RUN echo "deb [signed-by=/etc/apt/keyrings/mise-archive-keyring.gpg arch=amd64] https://mise.jdx.dev/deb stable main" |  tee /etc/apt/sources.list.d/mise.list
RUN apt update
RUN apt install -y mise

RUN mise install node@22 && mise global node@22
ENV PATH="/root/.local/share/mise/shims:/root/.local/bin:$PATH"
RUN npm install -g pnpm@10.12.1

# Install Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y \
    --no-modify-path --default-toolchain 1.89.0
ENV PATH="/root/.cargo/bin:$PATH"

# CMD ["node", "--version"]
WORKDIR /app
COPY . .

ENV PUBLIC_BASE_API_URL=""
RUN cd /app/packages/app \
    && pnpm install \
    && pnpm run build
RUN cd /app/packages/backend \
    && cargo build --release

RUN mkdir -p /data && touch /data/sifintra.db

FROM gcr.io/distroless/cc-debian12
COPY --from=builder /app/packages/backend/target/release/backend /app/backend
COPY --from=builder /data /data

ENV DATABASE_URL="/data/sifintra.db"
ENV HOST="0.0.0.0"
ENTRYPOINT ["/app/backend"]
