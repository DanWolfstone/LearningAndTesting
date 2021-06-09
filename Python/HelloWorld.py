# importing
from math import *
import sys


print("hello World")
#/---------------- STRINGS ----------------/#
# .lower()      || .upper()     # to make @a lowercase or @a uppercase 
# .islower()    || .isupper()   # 
# len(<str>)                    # Find out the length of a string
# <str>[x]                      # py equiv of .substring (assuming using other index tricks work as well)  
# .index(str)                   # py equiv of .indexOf, find the index of specified char or phrase (output = start of index) +lists
# .replace(str1, str2)          # replace char/phrase with another
# str(<var>)                    # converts variable to string
# int(<var>)    || float(<var>) # converts variable to int (whole#) or float (inf#)
# note: You can also use the == and != operators with strings

#/---------------- Numbers ----------------/#  // > from math import
# abs(#)                        # Absolute Value of a #
# pow(x, y)     || sqrt(#)      # Power funct and Sqrt funct  
# max(x, y)     || min(x, y)    # Max and min of the two numbers given
# round(#)                      # Standard rounding rules ^+.5 v-.5
# floor(#)      || ceil(#)      # floor = lowest int, ceil = highest int 

#getting user input
#name = input("Enter Your name:")
#age = input("Enter Your Age:") #input is automatically converted to string
age = "5"
print("You are " + age)         #Print can't display both ints/floats and strings without converting one
#print(int(age))                # Can also convert to str or float




#       |          *      ______      ________      ______ 
#       |          |      |____           |         |____
#       |____      |      _____|          |         _____|
#/---------------------------------------------------------/#
#                  0            1               2               3              4
listTest = ["List Item 1", "List Item 2", "List Item 3", "List Item 4", "List Item 5"]
NumListTest = [10, 20, 30, 40, 50]
# .extend(<list2>)                      # Extends og list with conts of list2, var type unimportant(?)
# .append(<var>)                        # Adds var to end of og list
# .insert(indx,<var>)                   # Adds var to indx pos, pushes array right 1,
# .remove(<var>)        || .pop()       # Removes specified var from list // removes last var
# .clear()              || .copy()      # Clears list or copies list
# .count(<var>)                         # how many times var appears in list
# .sort()               || .reverse()   # sorts/reverses in alphabetical/numerical order, 

print(listTest[1])      # Print by index
# print(ListTest)       // Prints entire list
# print(ListTest[1:])   // Prints everything past index[1]
# print(listTest[1:4])  // Prints everything from 1 to 3
listTest[2] = "List Item 3 Modified"


#/---------------- Tuples ----------------/#
# Like a list but can't be changed or modified, akin to FINAL in java(?)
# index      0    1
tupleTest = (10, 25)
# index             0          1         2
tupleListTest = [(10, 25), (18, 21), (55, 63)]
print(tupleListTest[1]) # || print(tupleTest[1]) 







#/---------------- FUNCTIONS ----------------/#
# indentions are important, only code indented after the semicolon will go in the funct 

def greeting(): #{
    print("Hello User")
#}

#param test
def stringParam(str):
    print("hello " + str + "!")
#Can you use an input in the funct call?
#stringParam(input("Enter your name:")) #yes you can! 

# testing multiple 
def multiParam(str, num):
    print("string input: " + str)
    print(num)

multiParam("hi", 15) # This works
# multiParam(input("Enter a string: "), input("Enter a Num: ")) #this also works

#Return test
def cube(num):
    return num*num*num
    #No code can execute after the return statement
    #No output unless a print or return is specified

print(cube(15))



#/---------------- IF STATEMENTS ----------------/#
# Operator's List
# if        || else
# elif      || or 
# and       || not
thisTrue = True
thisTrue2 = False

# Basic if else, I think you still need return statements if no print
if (thisTrue == True and thisTrue2 == False): #cannot use || as the <or> operator, also cannot use &&/& as the <and> operator
    print("is true")
else:
    print("is false")

# Basic if, else if, and else.
if (thisTrue == False):
    print("testing thisTrue to be False")
elif (thisTrue == True):
    print("testing elif (Else if) thisTrue == True")
else:
    print("You shouldn't see this, but it's the else")

# Left off at 1:55:00