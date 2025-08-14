#!/bin/bash

diesel migration revert -n 100
diesel migration run
cargo test
