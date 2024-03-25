load(":lib.bzl", "cool_fn")

def hello():
    print(cool_fn(42, 10))
