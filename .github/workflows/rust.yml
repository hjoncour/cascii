name: Rust

on:
  pull_request:
    branches: [ master ]

env:
  CARGO_TERM_COLOR: always

jobs:
  build:

    runs-on: ubuntu-latest

    steps:
    - uses: actions/checkout@v3
    - name: Install dependencies
      run: |
        sudo apt-get update
        sudo apt-get install -y ffmpeg imagemagick
    - name: Run tests
      run: sh test.sh
