In C, attempting to read beyond the end of a data structure is
undefined behavior. You might get whatever is at the location in
memory that would correspond to that element in the data structure,
even though the memory doesn’t belong to that structure. This is
called a buffer overread and can lead to security vulnerabilities if an
attacker is able to manipulate the index in such a way as to read
data they shouldn’t be allowed to that is stored after the data
structure. To protect your program from this sort of vulnerability, if you try to
read an element at an index that doesn’t exist, Rust will stop
execution and refuse to continue
