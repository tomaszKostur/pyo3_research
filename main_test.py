import inspect

import module_written_in_rust
out = module_written_in_rust.sum_as_string(5, 20)
print(out)

print(module_written_in_rust.__doc__)
print(inspect.signature(module_written_in_rust.sum_as_string))

# lest do some inter-language error
try:
    out = module_written_in_rust.sum_as_string(-2, 6)
except OverflowError as e:
    print(f"Wrong usage of rust module raised an Exception. details:\n{e}")

# Example of function that  has access to module data
print("Getting module data")
print(module_written_in_rust.return_module_name())

# kwargs_example
module_written_in_rust.kwargs_example(a='aa', b='bb')

# Rust Class
rc = module_written_in_rust.RustImplemented(4)
print(rc.add_predefined(4))