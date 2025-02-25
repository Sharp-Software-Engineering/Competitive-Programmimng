'''

Github (Feel Free to add a star): https://github.com/Sharp-Software-Engineering/Competitive-Programmimng
Rust template: https://github.com/Sharp-Software-Engineering/Competitive-Programmimng/templates/io-templates/python/template.py

Content Posts (after the contest) Follow: https://www.youtube.com/@SharpSoftwareEngineering

Repository contents free to modify and use.

'''

import time
import math
from collections import Counter, defaultdict
import heapq
FIXED_RANDOM = int(time.time()*100000)

def readInt():
    return int(input())

def readInts():
    return [int(x) for x in input().split(" ")]

def readStr():
    return input()

def readStrs():
    return [x for x in input().split(" ")]

# You can use mk(number) to mask a number and prevent hacking
def mk(k):
    k+=FIXED_RANDOM
    k+=0x9e3779b97f4a7c15
    k=(k^(k>>30))*0xbf58476d1ce4e5b9
    k=(k^(k>>27))*0x94d049bb133111eb 
    return k^(k>>31)

def main():
    t = readInt()
    for _ in range(t):
      n = readInt()
      print(n)
main()