# Notes
## GPU Stuff
* An SSBO's operations are atomic, like Image Load/Store, meaning we have to use a memory barrier. https://www.khronos.org/opengl/wiki/Memory_Model#Incoherent_memory_access (see "External visibility")

## Error handling
I should stop returning just a string containing an error message. I should instead return a proper error.
