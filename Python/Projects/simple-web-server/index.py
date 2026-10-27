## simple web server
## Written by Senal Bulumulle


import socket


HOST = '127.0.0.1'
PORT = 8080

'''
Variables
------------
server_socket
client_socket
client_address
'''

server_socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
server_socket.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)

## binding the address and the port
server_socket.bind((HOST, PORT))

## make it listen to the socket and the tcp
server_socket.listen(1)

print(f"Senal Server is running on {HOST}: {PORT}")


while True: 
    client_socket, client_address = server_socket.accept()
    print(f"Connection from {client_address}")