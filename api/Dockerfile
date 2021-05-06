FROM rust

RUN mkdir -p /usr/src/api
WORKDIR /usr/src/app

# Copy our app into that directory
COPY . /usr/src/api

# Build our app
CMD ["cargo", "build"]