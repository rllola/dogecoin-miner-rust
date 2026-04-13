FROM debian:trixie-slim

WORKDIR /usr/src

RUN apt-get update && apt-get install -y ca-certificates wget && rm -rf /var/lib/apt/lists/*

RUN wget https://download.litecoin.org/litecoin-0.21.5/litecoin-0.21.5-x86_64-linux-gnu.tar.gz
RUN tar -xzvf litecoin-0.21.5-x86_64-linux-gnu.tar.gz
RUN chmod +x litecoin-0.21.5/bin/litecoind litecoin-0.21.5/bin/litecoin-cli
RUN ln -s /usr/src/litecoin-0.21.5/bin/litecoind /usr/bin/litecoind
RUN ln -s /usr/src/litecoin-0.21.5/bin/litecoin-cli /usr/bin/litecoin-cli

EXPOSE 9332 9333 19332 19335 19443 19444

ENTRYPOINT ["litecoind", "-printtoconsole"]
