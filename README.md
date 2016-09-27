# mock-payroll-comparison

Used in demonstration during a [Rust NYC Meetup](https://www.meetup.com/Rust-NYC/events/233756447/).

## Purpose

This repo is meant to show that writing a web service in Rust is straightforward and easy to do, 
despite Rust's reputation as a low level systems language. 

To demonstrate, I have made nearly identical Node and Rust services to use as a comparison, with the index.html file serving as the frontend.

Despite both the fact that both the Rust and Node services have test coverage, **only** the Rust service saves us from a fatal flow, 
calculating the payment amounts as numeric values instead of string concatenating it like the Node service. Even though this is an overly
simple example, it goes to show how much better a statically typed language (such as Rust) is at enforcing program correctness.
