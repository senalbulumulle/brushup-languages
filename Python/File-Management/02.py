'''
File Management

Written by Senal
'''



from pathlib import Path




# @docs
# Here, this is where we create a new variable called getPath that uses
# the Path feature from pathlib to point to a directory of path.


getPath = Path("/Users/$USER/Documents/")




# @docs
# If the path that is pointed to doesn't exist, then it will break the
# program, using the while True statement


while True: 
    if getPath == getPath: 
        print("The file path is found. So some action here")
        break

    else: 
        print("The file path is not found. The program is stopped")
        break
