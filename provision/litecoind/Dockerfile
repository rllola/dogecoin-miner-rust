FROM ubuntu:latest

RUN apt-get update

RUN apt-get -y install wget

WORKDIR /usr/src

RUN wget https://download.litecoin.org/litecoin-0.18.1/linux/litecoin-0.18.1-x86_64-linux-gnu.tar.gz
RUN tar -xzvf litecoin-0.18.1-x86_64-linux-gnu.tar.gz
RUN chmod +x litecoin-0.18.1/bin/litecoind litecoin-0.18.1/bin/litecoin-cli
RUN ln -s /usr/src/litecoin-0.18.1/bin/litecoind /usr/bin/litecoind
RUN ln -s /usr/src/litecoin-0.18.1/bin/litecoin-cli /usr/bin/litecoin-cli

EXPOSE 9332 9333 19332 19335 19443 19444

CMD ["litecoind", "-printtoconsole"]
