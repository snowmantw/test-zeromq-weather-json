FROM liuchong/rustup:nightly
EXPOSE 5556
RUN apt-get update
RUN apt-get install -y sudo automake pkg-config git libtool
RUN rustup default nightly-2016-09-21

# Need to build libzmq fron scratch...
RUN git clone https://github.com/zeromq/libzmq.git
WORKDIR /root/libzmq 
RUN ./autogen.sh
RUN ./configure
RUN make
RUN make install
RUN ldconfig

# Then to build the package
ADD ./ /opt/app
# The flag is for the rust-zmq binding
WORKDIR /opt/app
RUN cargo build --release --package iomet-reactor
CMD /opt/app/target/release/iomet-reactor

