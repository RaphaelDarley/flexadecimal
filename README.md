# Flexadecimal

This project is inspired by MAT 2019 question 6, which features questions about algorithms of flexadecimal numbers. I am not currently aware of any actual utility.
Conversion to and from Flexadecimal is currently done via usize, however, this will quickly become insufficient as the maximum value is more than 10^500, this is because the Flexadecimal is currently implemented as an array of 255 u8s which means it could hold 255 * 255! + 254 * 254! + ...

This crate currently supports addition overloading with flexadecimals, conversion from and to usize, and from a string, which supports hex.