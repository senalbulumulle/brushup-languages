## simple web server
## Written by Senal Bulumulle


import socket


HOST = '127.0.0.1'
PORT = 8080


server_socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
server_socket.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)


server_socket.bind((HOST, PORT))