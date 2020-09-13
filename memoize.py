#!/usr/bin/env python
# Chapter 2, Challenge 1: Memoize function in Python

import time

def memoize(f):
    mappings = {}
    def memoized_f(x):
        if x not in mappings:
            mappings[x] = f(x)
        return mappings[x]
    return memoized_f

def long_running_function(x):
    time.sleep(5)
    return True

if __name__ == "__main__":
    memoized_function = memoize(long_running_function)

    start_time = time.time()
    print(f"memoized_function(1) = {memoized_function(1)}")
    print(f"duration = {time.time() - start_time}")

    start_time = time.time()
    print(f"memoized_function(1) = {memoized_function(1)}")
    print(f"duration = {time.time() - start_time}")
