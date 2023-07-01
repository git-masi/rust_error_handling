# Handling Errors in Rust HTTP Requests

## About

This repo is the companion to an article on the subject of error handling in Rust.

## Why

In Rust errors are values and conscious decisions need to be made about how to handle them through the application.

I think this can be challenging for people who primarily work in languages that throw exceptions so I wanted to create a case study to explore error handling.

This application is a CLI tool that will make HTTP requests with various levels of error handling. Run some examples and see what happens, then explore the code for the examples to get a sense for how error handling words.

## Getting started

To start the CLI simply use `cargo run` and then select the example HTTP request you want to make.

Note that **some examples will panic** and terminate the program. That is by design.
