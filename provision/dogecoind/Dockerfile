FROM debian:trixie-slim

WORKDIR /usr/src

RUN apt-get update && apt-get install -y ca-certificates wget && rm -rf /var/lib/apt/lists/*

RUN wget https://github.com/dogecoin/dogecoin/releases/download/v1.14.9/dogecoin-1.14.9-x86_64-linux-gnu.tar.gz
RUN tar -xzvf dogecoin-1.14.9-x86_64-linux-gnu.tar.gz
RUN chmod +x dogecoin-1.14.9/bin/dogecoind dogecoin-1.14.9/bin/dogecoin-cli
RUN ln -s /usr/src/dogecoin-1.14.9/bin/dogecoind /usr/bin/dogecoind
RUN ln -s /usr/src/dogecoin-1.14.9/bin/dogecoin-cli /usr/bin/dogecoin-cli

EXPOSE 22555 22556 44555 44556 18332 18444

ENTRYPOINT ["dogecoind", "-printtoconsole"]
