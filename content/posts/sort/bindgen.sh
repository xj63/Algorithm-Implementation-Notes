#!/bin/sh

bindgen ./c-src/sort.h > ./src/csort_bind.rs
