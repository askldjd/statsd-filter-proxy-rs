import socket
import sys

# Create a UDP socket
sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)

server_address = ('localhost', 8125)

try:
    num_send = 0

    while True:
        # Send data
        message = f'This is the message.  It will be repeated. {num_send}'
        print(f'sending {message}')
        sent = sock.sendto(str.encode(message), server_address)
        num_send += 1

        if num_send > 100:
            break
        

    # # Receive response
    # print(sys.stderr, 'waiting to receive')
    # print('waiting to receive')
    # data, server = sock.recvfrom(4096)
    # print(f'received {data}')

finally:
    print('closing socket')
    sock.close()