'''
This is a simple client script who will run the Rust binary library.

@author: Davide Aversa <thek3nger@gmail.com>

NOTE: Before using this script you neet to compile the rust library. Do that
with the command

    cargo build --release
    
This script is for Windows. Linux version will come.
'''

# Import the stadard interface library between Python and C libraries.
import ctypes

# We load the dll using WinDLL (windows format). For Linux try to comment
# this line and then uncomment the next one. 
lib = ctypes.WinDLL(".\\target\\debug\\rustypython.dll")
#lib = cdll.LoadLib(".\target\debug\rustypython.dll")

# We execute the Rust process() function we defined in src/lib.rs
print("Running process()")
lib.process()

lib.sum_list.argtypes = (ctypes.POINTER(ctypes.c_int32), ctypes.c_size_t)
print("Summing in Rust the list of first 1000 numbers.")
number_list = list(range(1001))
c_number_list = (ctypes.c_int32 * len(number_list))(*number_list)
result = lib.sum_list(c_number_list, len(number_list))
print("Result is {}. Expected 500500.".format(result))

# Congratulations!
print("done")