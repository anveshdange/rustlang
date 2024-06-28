#!/usr/bin/env python
""" 
This module is written for running rust code via a terminal 
"""

# importing modules
import os  
import sys 
from typing import List 
from typing import Optional 

__DIR_NAME__ : str = "targets"

# argument taking function 
def take_arg() -> str :
    print(f"Reading the file {sys.argv[1]}")
    return sys.argv[1]

# create folder in the workspace if not there 
def fldr(p) -> None:
    if(not os.path.isdir(__DIR_NAME__)) : 
        os.makedirs(__DIR_NAME__)
    run_str = f"rustc {p} -o ./{__DIR_NAME__}/out && ./{__DIR_NAME__}/out"
    os.system(run_str)
    return None 

# intermediate main function 
def main_func() -> None :
    # 1. Take argument of file name into the python file 
    p = take_arg()
    # 2. create a folder in current working directory and run everything
    fldr(p) 
    return None  

# DRIVER CODE 
if __name__ == "__main__" :
    main_func() # the main function call